//! MGGA_X_GVT4 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gvt4.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gvt4_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRTPI);
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = v_rho0 + v_rho1;
            let t6 = f64x8::splat(1.0) / t5;
            let t9 = (f64x8::splat(2.0) * v_rho0 * t6).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t13 = (f64x8::splat(2.0) * v_rho1 * t6).simd_le(zeta_threshold);
            let t14 = -t10;
            let t15 = v_rho0 - v_rho1;
            let t17 = ((t9).select(t10, (t13).select(t14, t15 * t6)));
            let t18 = f64x8::splat(1.0) + t17;
            let t19 = (t18).simd_le(zeta_threshold);
            let t20 = (simd::cbrt(zeta_threshold));
            let t21 = t20 * zeta_threshold;
            let t22 = (simd::cbrt(t18));
            let t24 = ((t19).select(t21, t22 * t18));
            let t25 = t4 * t24;
            let t26 = (simd::cbrt(t5));
            let t27 = t25 * t26;
            let t28 = v_rho0 * v_rho0;
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / t28;
            let t33 = v_sigma0 * t32;
            let t36 = f64x8::splat(1.0) / t30 / v_rho0;
            let t37 = v_tau0 * t36;
            let t39 = f64x8::splat(M_CBRT6);
            let t40 = t39 * t39;
            let t41 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t42 = (simd::cbrt(t41));
            let t43 = t42 * t42;
            let t44 = t40 * t43;
            let t45 = f64x8::splat(0.001120356) * t44;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.00186726) * t33 + f64x8::splat(0.00373452) * t37 - t45;
            let t51 = f64x8::splat(0.0037501956) * t44;
            let t52 = -f64x8::splat(0.003556788) * t33 + f64x8::splat(0.012500652) * t37 - t51;
            let t53 = t46 * t46;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = v_sigma0 * v_sigma0;
            let t57 = t28 * t28;
            let t58 = t57 * v_rho0;
            let t60 = f64x8::splat(1.0) / t29 / t58;
            let t64 = f64x8::splat(3.0) / f64x8::splat(5.0) * t44;
            let t65 = f64x8::splat(2.0) * t37 - t64;
            let t68 = t65 * t65;
            let t70 = -f64x8::splat(2.354518e-05) * t56 * t60 - f64x8::splat(0.0001282732) * t33 * t65 + f64x8::splat(0.0003574822) * t68;
            let t71 = t53 * t46;
            let t72 = f64x8::splat(1.0) / t71;
            let t76 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = f64x8::splat(M_CBRT4);
            let t80 = (-f64x8::splat(0.9800683) / t46 + t52 * t54 + t70 * t72) * t77 * t79;
            let t83 = ((t2).select(f64x8::splat(0.0), t27 * t80 / f64x8::splat(4.0)));
            let t84 = (v_rho1).simd_le(dens_threshold);
            let t85 = -t15;
            let t87 = ((t13).select(t10, (t9).select(t14, t85 * t6)));
            let t88 = f64x8::splat(1.0) + t87;
            let t89 = (t88).simd_le(zeta_threshold);
            let t90 = (simd::cbrt(t88));
            let t92 = ((t89).select(t21, t90 * t88));
            let t93 = t4 * t92;
            let t94 = t93 * t26;
            let t95 = v_rho1 * v_rho1;
            let t96 = (simd::cbrt(v_rho1));
            let t97 = t96 * t96;
            let t99 = f64x8::splat(1.0) / t97 / t95;
            let t100 = v_sigma2 * t99;
            let t103 = f64x8::splat(1.0) / t97 / v_rho1;
            let t104 = v_tau1 * t103;
            let t106 = f64x8::splat(1.0) + f64x8::splat(0.00186726) * t100 + f64x8::splat(0.00373452) * t104 - t45;
            let t111 = -f64x8::splat(0.003556788) * t100 + f64x8::splat(0.012500652) * t104 - t51;
            let t112 = t106 * t106;
            let t113 = f64x8::splat(1.0) / t112;
            let t115 = v_sigma2 * v_sigma2;
            let t116 = t95 * t95;
            let t117 = t116 * v_rho1;
            let t119 = f64x8::splat(1.0) / t96 / t117;
            let t123 = f64x8::splat(2.0) * t104 - t64;
            let t126 = t123 * t123;
            let t128 = -f64x8::splat(2.354518e-05) * t115 * t119 - f64x8::splat(0.0001282732) * t100 * t123 + f64x8::splat(0.0003574822) * t126;
            let t129 = t112 * t106;
            let t130 = f64x8::splat(1.0) / t129;
            let t134 = (-f64x8::splat(0.9800683) / t106 + t111 * t113 + t128 * t130) * t77 * t79;
            let t137 = ((t84).select(f64x8::splat(0.0), t94 * t134 / f64x8::splat(4.0)));
            let tzk0 = t83 + t137;
            acc_zk = tzk0;
            let t138 = t5 * t5;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t15 * t139;
            let t142 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), t6 - t140)));
            let t145 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t142));
            let t146 = t4 * t145;
            let t147 = t146 * t26;
            let t150 = t26 * t26;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t25 * t151;
            let t154 = t152 * t80 / f64x8::splat(12.0);
            let t155 = t28 * v_rho0;
            let t157 = f64x8::splat(1.0) / t30 / t155;
            let t158 = v_sigma0 * t157;
            let t160 = v_tau0 * t32;
            let t162 = -f64x8::splat(0.00497936) * t158 - f64x8::splat(0.0062242) * t160;
            let t167 = f64x8::splat(0.009484768) * t158 - f64x8::splat(0.02083442) * t160;
            let t169 = t52 * t72;
            let t172 = t57 * t28;
            let t174 = f64x8::splat(1.0) / t29 / t172;
            let t179 = v_sigma0 * t60;
            let t182 = t65 * v_tau0;
            let t185 = f64x8::splat(0.00012557429333333333) * t56 * t174 + f64x8::splat(0.00034206186666666666) * t158 * t65 + f64x8::splat(0.0004275773333333333) * t179 * v_tau0 - f64x8::splat(0.0023832146666666666) * t182 * t32;
            let t187 = t53 * t53;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t70 * t188;
            let t194 = (f64x8::splat(0.9800683) * t54 * t162 + t167 * t54 - f64x8::splat(2.0) * t169 * t162 + t185 * t72 - f64x8::splat(3.0) * t189 * t162) * t77 * t79;
            let t198 = ((t2).select(f64x8::splat(0.0), t147 * t80 / f64x8::splat(4.0) + t154 + t27 * t194 / f64x8::splat(4.0)));
            let t199 = t85 * t139;
            let t201 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), -t6 - t199)));
            let t204 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t201));
            let t205 = t4 * t204;
            let t206 = t205 * t26;
            let t209 = t93 * t151;
            let t211 = t209 * t134 / f64x8::splat(12.0);
            let t213 = ((t84).select(f64x8::splat(0.0), t206 * t134 / f64x8::splat(4.0) + t211));
            let tvrho0 = t83 + t137 + t5 * (t198 + t213);
            acc_vrho_0 = tvrho0;
            let t217 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), -t6 - t140)));
            let t220 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t217));
            let t221 = t4 * t220;
            let t222 = t221 * t26;
            let t226 = ((t2).select(f64x8::splat(0.0), t222 * t80 / f64x8::splat(4.0) + t154));
            let t228 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), t6 - t199)));
            let t231 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t228));
            let t232 = t4 * t231;
            let t233 = t232 * t26;
            let t236 = t95 * v_rho1;
            let t238 = f64x8::splat(1.0) / t97 / t236;
            let t239 = v_sigma2 * t238;
            let t241 = v_tau1 * t99;
            let t243 = -f64x8::splat(0.00497936) * t239 - f64x8::splat(0.0062242) * t241;
            let t248 = f64x8::splat(0.009484768) * t239 - f64x8::splat(0.02083442) * t241;
            let t250 = t111 * t130;
            let t253 = t116 * t95;
            let t255 = f64x8::splat(1.0) / t96 / t253;
            let t260 = v_sigma2 * t119;
            let t263 = t123 * v_tau1;
            let t266 = f64x8::splat(0.00012557429333333333) * t115 * t255 + f64x8::splat(0.00034206186666666666) * t239 * t123 + f64x8::splat(0.0004275773333333333) * t260 * v_tau1 - f64x8::splat(0.0023832146666666666) * t263 * t99;
            let t268 = t112 * t112;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t128 * t269;
            let t275 = (f64x8::splat(0.9800683) * t113 * t243 + t248 * t113 - f64x8::splat(2.0) * t250 * t243 + t266 * t130 - f64x8::splat(3.0) * t270 * t243) * t77 * t79;
            let t279 = ((t84).select(f64x8::splat(0.0), t233 * t134 / f64x8::splat(4.0) + t211 + t94 * t275 / f64x8::splat(4.0)));
            let tvrho1 = t83 + t137 + t5 * (t226 + t279);
            acc_vrho_1 = tvrho1;
            let t282 = t54 * t32;
            let t284 = t169 * t32;
            let t287 = t32 * t65;
            let t289 = -f64x8::splat(4.709036e-05) * t179 - f64x8::splat(0.0001282732) * t287;
            let t291 = t189 * t32;
            let t295 = (-f64x8::splat(0.001726745666142) * t282 - f64x8::splat(0.00373452) * t284 + t289 * t72 - f64x8::splat(0.00560178) * t291) * t77 * t79;
            let t298 = ((t2).select(f64x8::splat(0.0), t27 * t295 / f64x8::splat(4.0)));
            let tvsigma0 = t5 * t298;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t299 = t113 * t99;
            let t301 = t250 * t99;
            let t304 = t99 * t123;
            let t306 = -f64x8::splat(4.709036e-05) * t260 - f64x8::splat(0.0001282732) * t304;
            let t308 = t270 * t99;
            let t312 = (-f64x8::splat(0.001726745666142) * t299 - f64x8::splat(0.00373452) * t301 + t306 * t130 - f64x8::splat(0.00560178) * t308) * t77 * t79;
            let t315 = ((t84).select(f64x8::splat(0.0), t94 * t312 / f64x8::splat(4.0)));
            let tvsigma2 = t5 * t315;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t321 = f64x8::splat(1.0) / t29 / t57;
            let t326 = -f64x8::splat(0.0002565464) * v_sigma0 * t321 + f64x8::splat(0.0014299288) * t65 * t36;
            let t332 = (f64x8::splat(0.016160736667716) * t54 * t36 - f64x8::splat(0.00746904) * t169 * t36 + t326 * t72 - f64x8::splat(0.01120356) * t189 * t36) * t77 * t79;
            let t335 = ((t2).select(f64x8::splat(0.0), t27 * t332 / f64x8::splat(4.0)));
            let tvtau0 = t5 * t335;
            acc_vtau_0 = tvtau0;
            let t341 = f64x8::splat(1.0) / t96 / t116;
            let t346 = -f64x8::splat(0.0002565464) * v_sigma2 * t341 + f64x8::splat(0.0014299288) * t123 * t103;
            let t352 = (f64x8::splat(0.016160736667716) * t113 * t103 - f64x8::splat(0.00746904) * t250 * t103 + t346 * t130 - f64x8::splat(0.01120356) * t270 * t103) * t77 * t79;
            let t355 = ((t84).select(f64x8::splat(0.0), t94 * t352 / f64x8::splat(4.0)));
            let tvtau1 = t5 * t355;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
