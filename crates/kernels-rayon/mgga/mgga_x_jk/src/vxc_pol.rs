//! MGGA_X_JK vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`
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
pub fn mgga_x_jk_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = t3 * t3;
            let t30 = param_beta * t29;
            let t32 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = f64x8::splat(M_CBRT4);
            let t35 = t33 * t34;
            let t36 = t30 * t35;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t40 = t39 * t37;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = v_sigma0 * t41;
            let t43 = param_gamma * param_beta;
            let t44 = ((v_sigma0).sqrt());
            let t45 = t38 * v_rho0;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t44 * t46;
            let t48 = (simd::ln(t47 + ((t47 * t47 + f64x8::splat(1.0)).sqrt())));
            let t51 = t43 * t47 * t48 + f64x8::splat(1.0);
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t39 * v_rho0;
            let t54 = f64x8::splat(1.0) / t53;
            let t56 = -v_lapl0 * t54 + t42;
            let t57 = f64x8::splat(1.0) / v_sigma0;
            let t58 = t56 * t57;
            let t61 = f64x8::splat(2.0) * t40 * t58 + f64x8::splat(1.0);
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t52 * t62;
            let t67 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t42 * t63;
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t67));
            let t72 = (v_rho1).simd_le(dens_threshold);
            let t73 = -t17;
            let t75 = ((t15).select(t12, (t11).select(t16, t73 * t8)));
            let t76 = f64x8::splat(1.0) + t75;
            let t77 = (t76).simd_le(zeta_threshold);
            let t78 = (simd::cbrt(t76));
            let t80 = ((t77).select(t23, t78 * t76));
            let t81 = t80 * t27;
            let t82 = v_rho1 * v_rho1;
            let t83 = (simd::cbrt(v_rho1));
            let t84 = t83 * t83;
            let t85 = t84 * t82;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = v_sigma2 * t86;
            let t88 = ((v_sigma2).sqrt());
            let t89 = t83 * v_rho1;
            let t90 = f64x8::splat(1.0) / t89;
            let t91 = t88 * t90;
            let t92 = (simd::ln(t91 + ((t91 * t91 + f64x8::splat(1.0)).sqrt())));
            let t95 = t43 * t91 * t92 + f64x8::splat(1.0);
            let t96 = f64x8::splat(1.0) / t95;
            let t97 = t84 * v_rho1;
            let t98 = f64x8::splat(1.0) / t97;
            let t100 = -v_lapl1 * t98 + t87;
            let t101 = f64x8::splat(1.0) / v_sigma2;
            let t102 = t100 * t101;
            let t105 = f64x8::splat(2.0) * t102 * t85 + f64x8::splat(1.0);
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t96 * t106;
            let t111 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t87 * t107;
            let t115 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t81 * t111));
            let tzk0 = t71 + t115;
            acc_zk = tzk0;
            let t116 = t7 * t7;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t17 * t117;
            let t120 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t118)));
            let t123 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t120));
            let t124 = t123 * t27;
            let t128 = t27 * t27;
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t26 * t129;
            let t133 = t6 * t130 * t67 / f64x8::splat(8.0);
            let t134 = t37 * v_rho0;
            let t136 = f64x8::splat(1.0) / t39 / t134;
            let t137 = v_sigma0 * t136;
            let t141 = t51 * t51;
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t142 * t62;
            let t145 = f64x8::splat(1.0) / t38 / t37;
            let t149 = t42 + f64x8::splat(1.0);
            let t150 = ((t149).sqrt());
            let t151 = f64x8::splat(1.0) / t150;
            let t155 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t44 * t145 * t48 - f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t137 * t151;
            let t156 = t143 * t155;
            let t160 = t61 * t61;
            let t161 = f64x8::splat(1.0) / t160;
            let t162 = t52 * t161;
            let t166 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t137 + f64x8::splat(5.0) / f64x8::splat(3.0) * v_lapl0 * t41;
            let t167 = t166 * t57;
            let t172 = f64x8::splat(2.0) * t167 * t40 + f64x8::splat(16.0) / f64x8::splat(3.0) * t58 * t53;
            let t173 = t162 * t172;
            let t177 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t36 * t137 * t63 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t42 * t156 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t42 * t173;
            let t182 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t124 * t67 - t133 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t177));
            let t183 = t73 * t117;
            let t185 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t183)));
            let t188 = ((t77).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t78 * t185));
            let t189 = t188 * t27;
            let t193 = t80 * t129;
            let t196 = t6 * t193 * t111 / f64x8::splat(8.0);
            let t198 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t189 * t111 - t196));
            let tvrho0 = t71 + t115 + t7 * (t182 + t198);
            acc_vrho_0 = tvrho0;
            let t202 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t118)));
            let t205 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t202));
            let t206 = t205 * t27;
            let t211 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t206 * t67 - t133));
            let t213 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t183)));
            let t216 = ((t77).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t78 * t213));
            let t217 = t216 * t27;
            let t221 = t82 * v_rho1;
            let t223 = f64x8::splat(1.0) / t84 / t221;
            let t224 = v_sigma2 * t223;
            let t228 = t95 * t95;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t229 * t106;
            let t232 = f64x8::splat(1.0) / t83 / t82;
            let t236 = t87 + f64x8::splat(1.0);
            let t237 = ((t236).sqrt());
            let t238 = f64x8::splat(1.0) / t237;
            let t242 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t88 * t232 * t92 - f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t224 * t238;
            let t243 = t230 * t242;
            let t247 = t105 * t105;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t96 * t248;
            let t253 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t224 + f64x8::splat(5.0) / f64x8::splat(3.0) * v_lapl1 * t86;
            let t254 = t253 * t101;
            let t259 = f64x8::splat(2.0) * t254 * t85 + f64x8::splat(16.0) / f64x8::splat(3.0) * t102 * t97;
            let t260 = t249 * t259;
            let t264 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t36 * t224 * t107 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t87 * t243 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t87 * t260;
            let t269 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t217 * t111 - t196 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t81 * t264));
            let tvrho1 = t71 + t115 + t7 * (t211 + t269);
            acc_vrho_1 = tvrho1;
            let t272 = t30 * t33;
            let t273 = t34 * t41;
            let t276 = f64x8::splat(1.0) / t44;
            let t283 = t43 * t276 * t46 * t48 / f64x8::splat(2.0) + t43 * t41 * t151 / f64x8::splat(2.0);
            let t284 = t143 * t283;
            let t287 = v_sigma0 * v_sigma0;
            let t288 = f64x8::splat(1.0) / t287;
            let t289 = t56 * t288;
            let t292 = -f64x8::splat(2.0) * t289 * t40 + f64x8::splat(2.0) * t57;
            let t293 = t162 * t292;
            let t297 = f64x8::splat(2.0) / f64x8::splat(9.0) * t272 * t273 * t63 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t42 * t284 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t42 * t293;
            let t301 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t297));
            let tvsigma0 = t7 * t301;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t302 = t34 * t86;
            let t305 = f64x8::splat(1.0) / t88;
            let t312 = t43 * t305 * t90 * t92 / f64x8::splat(2.0) + t43 * t86 * t238 / f64x8::splat(2.0);
            let t313 = t230 * t312;
            let t316 = v_sigma2 * v_sigma2;
            let t317 = f64x8::splat(1.0) / t316;
            let t318 = t100 * t317;
            let t321 = -f64x8::splat(2.0) * t318 * t85 + f64x8::splat(2.0) * t101;
            let t322 = t249 * t321;
            let t326 = f64x8::splat(2.0) / f64x8::splat(9.0) * t272 * t302 * t107 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t87 * t313 - f64x8::splat(2.0) / f64x8::splat(9.0) * t36 * t87 * t322;
            let t330 = ((t72).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t81 * t326));
            let tvsigma2 = t7 * t330;
            acc_vsigma_2 = tvsigma2;
            let t331 = t5 * t26;
            let t332 = t27 * param_beta;
            let t333 = t331 * t332;
            let t336 = t35 * t54 * t52 * t161;
            let t339 = ((t2).select(f64x8::splat(0.0), -t333 * t336 / f64x8::splat(2.0)));
            let tvlapl0 = t7 * t339;
            acc_vlapl_0 = tvlapl0;
            let t340 = t5 * t80;
            let t341 = t340 * t332;
            let t344 = t35 * t98 * t96 * t248;
            let t347 = ((t72).select(f64x8::splat(0.0), -t341 * t344 / f64x8::splat(2.0)));
            let tvlapl1 = t7 * t347;
            acc_vlapl_1 = tvlapl1;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau_0 = tvtau0;
            let tvtau1 = f64x8::splat(0.0);
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
