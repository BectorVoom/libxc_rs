//! HYB_MGGA_X_M05 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_m05.c`
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
pub fn hyb_mgga_x_m05_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_csi_HF: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_a_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_csi_HF = f64x8::splat(param_csi_HF);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_a_6 = f64x8::splat(param_a_6);
    let param_a_7 = f64x8::splat(param_a_7);
    let param_a_8 = f64x8::splat(param_a_8);
    let param_a_9 = f64x8::splat(param_a_9);
    let param_a_10 = f64x8::splat(param_a_10);
    let param_a_11 = f64x8::splat(param_a_11);
    let param_a_0 = f64x8::splat(param_a_0);
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
            let t6 = t3 / t4;
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = t28 * param_csi_HF;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t30 * t34;
            let t36 = v_rho0 * v_rho0;
            let t37 = (simd::cbrt(v_rho0));
            let t38 = t37 * t37;
            let t40 = f64x8::splat(1.0) / t38 / t36;
            let t44 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t35 * v_sigma0 * t40;
            let t47 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t44;
            let t48 = param_a_0;
            let t49 = param_a_1;
            let t50 = t30 * t30;
            let t52 = f64x8::splat(3.0) / f64x8::splat(10.0) * t50 * t33;
            let t54 = f64x8::splat(1.0) / t38 / v_rho0;
            let t55 = v_tau0 * t54;
            let t56 = t52 - t55;
            let t57 = t49 * t56;
            let t58 = t52 + t55;
            let t59 = f64x8::splat(1.0) / t58;
            let t61 = param_a_2;
            let t62 = t56 * t56;
            let t63 = t61 * t62;
            let t64 = t58 * t58;
            let t65 = f64x8::splat(1.0) / t64;
            let t67 = param_a_3;
            let t68 = t62 * t56;
            let t69 = t67 * t68;
            let t70 = t64 * t58;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = param_a_4;
            let t74 = t62 * t62;
            let t75 = t73 * t74;
            let t76 = t64 * t64;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = param_a_5;
            let t80 = t74 * t56;
            let t81 = t79 * t80;
            let t82 = t76 * t58;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = param_a_6;
            let t86 = t74 * t62;
            let t87 = t85 * t86;
            let t88 = t76 * t64;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = param_a_7;
            let t92 = t74 * t68;
            let t93 = t91 * t92;
            let t94 = t76 * t70;
            let t95 = f64x8::splat(1.0) / t94;
            let t97 = param_a_8;
            let t98 = t74 * t74;
            let t99 = t97 * t98;
            let t100 = t76 * t76;
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = param_a_9;
            let t104 = t98 * t56;
            let t105 = t103 * t104;
            let t107 = f64x8::splat(1.0) / t100 / t58;
            let t109 = param_a_10;
            let t110 = t98 * t62;
            let t111 = t109 * t110;
            let t113 = f64x8::splat(1.0) / t100 / t64;
            let t115 = param_a_11;
            let t117 = t115 * t98 * t68;
            let t119 = f64x8::splat(1.0) / t100 / t70;
            let t121 = t99 * t101 + t105 * t107 + t111 * t113 + t117 * t119 + t57 * t59 + t63 * t65 + t69 * t71 + t75 * t77 + t81 * t83 + t87 * t89 + t93 * t95 + t48;
            let t122 = t47 * t121;
            let t123 = t29 * t122;
            let t126 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t123));
            let t127 = (v_rho1).simd_le(dens_threshold);
            let t128 = -t17;
            let t130 = ((t15).select(t12, (t11).select(t16, t128 * t8)));
            let t131 = f64x8::splat(1.0) + t130;
            let t132 = (t131).simd_le(zeta_threshold);
            let t133 = (simd::cbrt(t131));
            let t135 = ((t132).select(t23, t133 * t131));
            let t136 = t6 * t135;
            let t137 = v_rho1 * v_rho1;
            let t138 = (simd::cbrt(v_rho1));
            let t139 = t138 * t138;
            let t141 = f64x8::splat(1.0) / t139 / t137;
            let t145 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t35 * v_sigma2 * t141;
            let t148 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t145;
            let t150 = f64x8::splat(1.0) / t139 / v_rho1;
            let t151 = v_tau1 * t150;
            let t152 = t52 - t151;
            let t153 = t49 * t152;
            let t154 = t52 + t151;
            let t155 = f64x8::splat(1.0) / t154;
            let t157 = t152 * t152;
            let t158 = t61 * t157;
            let t159 = t154 * t154;
            let t160 = f64x8::splat(1.0) / t159;
            let t162 = t157 * t152;
            let t163 = t67 * t162;
            let t164 = t159 * t154;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = t157 * t157;
            let t168 = t73 * t167;
            let t169 = t159 * t159;
            let t170 = f64x8::splat(1.0) / t169;
            let t172 = t167 * t152;
            let t173 = t79 * t172;
            let t174 = t169 * t154;
            let t175 = f64x8::splat(1.0) / t174;
            let t177 = t167 * t157;
            let t178 = t85 * t177;
            let t179 = t169 * t159;
            let t180 = f64x8::splat(1.0) / t179;
            let t182 = t167 * t162;
            let t183 = t91 * t182;
            let t184 = t169 * t164;
            let t185 = f64x8::splat(1.0) / t184;
            let t187 = t167 * t167;
            let t188 = t97 * t187;
            let t189 = t169 * t169;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = t187 * t152;
            let t193 = t103 * t192;
            let t195 = f64x8::splat(1.0) / t189 / t154;
            let t197 = t187 * t157;
            let t198 = t109 * t197;
            let t200 = f64x8::splat(1.0) / t189 / t159;
            let t203 = t115 * t187 * t162;
            let t205 = f64x8::splat(1.0) / t189 / t164;
            let t207 = t153 * t155 + t158 * t160 + t163 * t165 + t168 * t170 + t173 * t175 + t178 * t180 + t183 * t185 + t188 * t190 + t193 * t195 + t198 * t200 + t203 * t205 + t48;
            let t208 = t148 * t207;
            let t209 = t29 * t208;
            let t212 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t136 * t209));
            let tzk0 = t126 + t212;
            acc_zk = tzk0;
            let t213 = t7 * t7;
            let t214 = f64x8::splat(1.0) / t213;
            let t215 = t17 * t214;
            let t217 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t215)));
            let t220 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t217));
            let t221 = t6 * t220;
            let t224 = t28 * t28;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t225 * param_csi_HF;
            let t227 = t226 * t122;
            let t229 = t27 * t227 / f64x8::splat(8.0);
            let t230 = t3 * t26;
            let t231 = t44 * t44;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t29 * t232;
            let t234 = t230 * t233;
            let t235 = t36 * v_rho0;
            let t237 = f64x8::splat(1.0) / t38 / t235;
            let t238 = v_sigma0 * t237;
            let t240 = t35 * t238 * t121;
            let t243 = t49 * v_tau0;
            let t247 = t65 * v_tau0;
            let t248 = t247 * t40;
            let t251 = t61 * t56;
            let t254 = t71 * v_tau0;
            let t255 = t254 * t40;
            let t258 = t67 * t62;
            let t261 = t77 * v_tau0;
            let t262 = t261 * t40;
            let t265 = t73 * t68;
            let t268 = t83 * v_tau0;
            let t269 = t268 * t40;
            let t272 = t79 * t74;
            let t275 = t89 * v_tau0;
            let t276 = t275 * t40;
            let t279 = t85 * t80;
            let t282 = f64x8::splat(5.0) / f64x8::splat(3.0) * t243 * t40 * t59 + f64x8::splat(5.0) / f64x8::splat(3.0) * t57 * t248 + f64x8::splat(10.0) / f64x8::splat(3.0) * t251 * t248 + f64x8::splat(10.0) / f64x8::splat(3.0) * t63 * t255 + f64x8::splat(5.0) * t258 * t255 + f64x8::splat(5.0) * t69 * t262 + f64x8::splat(20.0) / f64x8::splat(3.0) * t265 * t262 + f64x8::splat(20.0) / f64x8::splat(3.0) * t75 * t269 + f64x8::splat(25.0) / f64x8::splat(3.0) * t272 * t269 + f64x8::splat(25.0) / f64x8::splat(3.0) * t81 * t276 + f64x8::splat(10.0) * t279 * t276;
            let t283 = t95 * v_tau0;
            let t284 = t283 * t40;
            let t287 = t91 * t86;
            let t290 = t101 * v_tau0;
            let t291 = t290 * t40;
            let t294 = t97 * t92;
            let t297 = t107 * v_tau0;
            let t298 = t297 * t40;
            let t301 = t103 * t98;
            let t304 = t113 * v_tau0;
            let t305 = t304 * t40;
            let t308 = t109 * t104;
            let t311 = t119 * v_tau0;
            let t312 = t311 * t40;
            let t315 = t115 * t110;
            let t319 = f64x8::splat(1.0) / t100 / t76;
            let t320 = t319 * v_tau0;
            let t324 = f64x8::splat(10.0) * t87 * t284 + f64x8::splat(35.0) / f64x8::splat(3.0) * t287 * t284 + f64x8::splat(35.0) / f64x8::splat(3.0) * t93 * t291 + f64x8::splat(40.0) / f64x8::splat(3.0) * t294 * t291 + f64x8::splat(40.0) / f64x8::splat(3.0) * t99 * t298 + f64x8::splat(15.0) * t301 * t298 + f64x8::splat(15.0) * t105 * t305 + f64x8::splat(50.0) / f64x8::splat(3.0) * t308 * t305 + f64x8::splat(50.0) / f64x8::splat(3.0) * t111 * t312 + f64x8::splat(55.0) / f64x8::splat(3.0) * t315 * t312 + f64x8::splat(55.0) / f64x8::splat(3.0) * t117 * t320 * t40;
            let t325 = t282 + t324;
            let t326 = t47 * t325;
            let t327 = t29 * t326;
            let t331 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t221 * t123 - t229 + f64x8::splat(0.0040369036088841095) * t234 * t240 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t327));
            let t332 = t128 * t214;
            let t334 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t332)));
            let t337 = ((t132).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t133 * t334));
            let t338 = t6 * t337;
            let t341 = t226 * t208;
            let t343 = t136 * t341 / f64x8::splat(8.0);
            let t345 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t338 * t209 - t343));
            let tvrho0 = t126 + t212 + t7 * (t331 + t345);
            acc_vrho_0 = tvrho0;
            let t349 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t215)));
            let t352 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t349));
            let t353 = t6 * t352;
            let t357 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t353 * t123 - t229));
            let t359 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t332)));
            let t362 = ((t132).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t133 * t359));
            let t363 = t6 * t362;
            let t366 = t3 * t135;
            let t367 = t145 * t145;
            let t368 = f64x8::splat(1.0) / t367;
            let t369 = t29 * t368;
            let t370 = t366 * t369;
            let t371 = t137 * v_rho1;
            let t373 = f64x8::splat(1.0) / t139 / t371;
            let t374 = v_sigma2 * t373;
            let t376 = t35 * t374 * t207;
            let t379 = t49 * v_tau1;
            let t383 = t160 * v_tau1;
            let t384 = t383 * t141;
            let t387 = t61 * t152;
            let t390 = t165 * v_tau1;
            let t391 = t390 * t141;
            let t394 = t67 * t157;
            let t397 = t170 * v_tau1;
            let t398 = t397 * t141;
            let t401 = t73 * t162;
            let t404 = t175 * v_tau1;
            let t405 = t404 * t141;
            let t408 = t79 * t167;
            let t411 = t180 * v_tau1;
            let t412 = t411 * t141;
            let t415 = t85 * t172;
            let t418 = f64x8::splat(5.0) / f64x8::splat(3.0) * t379 * t141 * t155 + f64x8::splat(5.0) / f64x8::splat(3.0) * t153 * t384 + f64x8::splat(10.0) / f64x8::splat(3.0) * t387 * t384 + f64x8::splat(10.0) / f64x8::splat(3.0) * t158 * t391 + f64x8::splat(5.0) * t394 * t391 + f64x8::splat(5.0) * t163 * t398 + f64x8::splat(20.0) / f64x8::splat(3.0) * t401 * t398 + f64x8::splat(20.0) / f64x8::splat(3.0) * t168 * t405 + f64x8::splat(25.0) / f64x8::splat(3.0) * t408 * t405 + f64x8::splat(25.0) / f64x8::splat(3.0) * t173 * t412 + f64x8::splat(10.0) * t415 * t412;
            let t419 = t185 * v_tau1;
            let t420 = t419 * t141;
            let t423 = t91 * t177;
            let t426 = t190 * v_tau1;
            let t427 = t426 * t141;
            let t430 = t97 * t182;
            let t433 = t195 * v_tau1;
            let t434 = t433 * t141;
            let t437 = t103 * t187;
            let t440 = t200 * v_tau1;
            let t441 = t440 * t141;
            let t444 = t109 * t192;
            let t447 = t205 * v_tau1;
            let t448 = t447 * t141;
            let t451 = t115 * t197;
            let t455 = f64x8::splat(1.0) / t189 / t169;
            let t456 = t455 * v_tau1;
            let t460 = f64x8::splat(10.0) * t178 * t420 + f64x8::splat(35.0) / f64x8::splat(3.0) * t423 * t420 + f64x8::splat(35.0) / f64x8::splat(3.0) * t183 * t427 + f64x8::splat(40.0) / f64x8::splat(3.0) * t430 * t427 + f64x8::splat(40.0) / f64x8::splat(3.0) * t188 * t434 + f64x8::splat(15.0) * t437 * t434 + f64x8::splat(15.0) * t193 * t441 + f64x8::splat(50.0) / f64x8::splat(3.0) * t444 * t441 + f64x8::splat(50.0) / f64x8::splat(3.0) * t198 * t448 + f64x8::splat(55.0) / f64x8::splat(3.0) * t451 * t448 + f64x8::splat(55.0) / f64x8::splat(3.0) * t203 * t456 * t141;
            let t461 = t418 + t460;
            let t462 = t148 * t461;
            let t463 = t29 * t462;
            let t467 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t363 * t209 - t343 + f64x8::splat(0.0040369036088841095) * t370 * t376 - f64x8::splat(3.0) / f64x8::splat(8.0) * t136 * t463));
            let tvrho1 = t126 + t212 + t7 * (t357 + t467);
            acc_vrho_1 = tvrho1;
            let t470 = t230 * t29;
            let t471 = t232 * t30;
            let t472 = t34 * t40;
            let t474 = t471 * t472 * t121;
            let t477 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0015138388533315413) * t470 * t474));
            let tvsigma0 = t7 * t477;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t478 = t366 * t29;
            let t479 = t368 * t30;
            let t480 = t34 * t141;
            let t482 = t479 * t480 * t207;
            let t485 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(0.0015138388533315413) * t478 * t482));
            let tvsigma2 = t7 * t485;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t488 = t65 * t54;
            let t492 = t71 * t54;
            let t497 = t77 * t54;
            let t502 = t83 * t54;
            let t507 = t89 * t54;
            let t512 = -t49 * t54 * t59 - f64x8::splat(2.0) * t251 * t488 - f64x8::splat(3.0) * t258 * t492 - f64x8::splat(4.0) * t265 * t497 - f64x8::splat(5.0) * t272 * t502 - f64x8::splat(6.0) * t279 * t507 - t57 * t488 - f64x8::splat(2.0) * t63 * t492 - f64x8::splat(3.0) * t69 * t497 - f64x8::splat(4.0) * t75 * t502 - f64x8::splat(5.0) * t81 * t507;
            let t513 = t95 * t54;
            let t518 = t101 * t54;
            let t523 = t107 * t54;
            let t528 = t113 * t54;
            let t533 = t119 * t54;
            let t541 = -f64x8::splat(11.0) * t117 * t319 * t54 - f64x8::splat(9.0) * t105 * t528 - f64x8::splat(10.0) * t111 * t533 - f64x8::splat(7.0) * t287 * t513 - f64x8::splat(8.0) * t294 * t518 - f64x8::splat(9.0) * t301 * t523 - f64x8::splat(10.0) * t308 * t528 - f64x8::splat(11.0) * t315 * t533 - f64x8::splat(6.0) * t87 * t513 - f64x8::splat(7.0) * t93 * t518 - f64x8::splat(8.0) * t99 * t523;
            let t542 = t512 + t541;
            let t543 = t47 * t542;
            let t544 = t29 * t543;
            let t547 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t544));
            let tvtau0 = t7 * t547;
            acc_vtau_0 = tvtau0;
            let t550 = t160 * t150;
            let t554 = t165 * t150;
            let t559 = t170 * t150;
            let t564 = t175 * t150;
            let t569 = t180 * t150;
            let t574 = -t49 * t150 * t155 - t153 * t550 - f64x8::splat(2.0) * t158 * t554 - f64x8::splat(3.0) * t163 * t559 - f64x8::splat(4.0) * t168 * t564 - f64x8::splat(5.0) * t173 * t569 - f64x8::splat(2.0) * t387 * t550 - f64x8::splat(3.0) * t394 * t554 - f64x8::splat(4.0) * t401 * t559 - f64x8::splat(5.0) * t408 * t564 - f64x8::splat(6.0) * t415 * t569;
            let t575 = t185 * t150;
            let t580 = t190 * t150;
            let t585 = t195 * t150;
            let t590 = t200 * t150;
            let t595 = t205 * t150;
            let t603 = -f64x8::splat(11.0) * t203 * t455 * t150 - f64x8::splat(6.0) * t178 * t575 - f64x8::splat(7.0) * t183 * t580 - f64x8::splat(8.0) * t188 * t585 - f64x8::splat(9.0) * t193 * t590 - f64x8::splat(10.0) * t198 * t595 - f64x8::splat(7.0) * t423 * t575 - f64x8::splat(8.0) * t430 * t580 - f64x8::splat(9.0) * t437 * t585 - f64x8::splat(10.0) * t444 * t590 - f64x8::splat(11.0) * t451 * t595;
            let t604 = t574 + t603;
            let t605 = t148 * t604;
            let t606 = t29 * t605;
            let t609 = ((t127).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t136 * t606));
            let tvtau1 = t7 * t609;
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
