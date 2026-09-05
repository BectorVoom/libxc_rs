//! GGA_X_B86 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_b86_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
    let param_omega = f64x8::splat(param_omega);
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_beta * v_sigma0;
            let t29 = v_rho0 * v_rho0;
            let t30 = (simd::cbrt(v_rho0));
            let t31 = t30 * t30;
            let t33 = f64x8::splat(1.0) / t31 / t29;
            let t36 = param_gamma * v_sigma0 * t33 + f64x8::splat(1.0);
            let t37 = (simd::pow(t36, param_omega));
            let t38 = f64x8::splat(1.0) / t37;
            let t41 = t28 * t33 * t38 + f64x8::splat(1.0);
            let t45 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t41));
            let t46 = (v_rho1).simd_le(dens_threshold);
            let t47 = -t16;
            let t49 = ((t14).select(t11, (t10).select(t15, t47 * t7)));
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t54 = ((t51).select(t22, t52 * t50));
            let t55 = t54 * t26;
            let t56 = param_beta * v_sigma2;
            let t57 = v_rho1 * v_rho1;
            let t58 = (simd::cbrt(v_rho1));
            let t59 = t58 * t58;
            let t61 = f64x8::splat(1.0) / t59 / t57;
            let t64 = param_gamma * v_sigma2 * t61 + f64x8::splat(1.0);
            let t65 = (simd::pow(t64, param_omega));
            let t66 = f64x8::splat(1.0) / t65;
            let t69 = t56 * t61 * t66 + f64x8::splat(1.0);
            let t73 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t69));
            let tzk0 = t45 + t73;
            acc_zk = tzk0;
            let t74 = t6 * t6;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t16 * t75;
            let t78 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t76)));
            let t81 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t78));
            let t82 = t81 * t26;
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t25 * t87;
            let t91 = t5 * t88 * t41 / f64x8::splat(8.0);
            let t92 = t29 * v_rho0;
            let t94 = f64x8::splat(1.0) / t31 / t92;
            let t97 = v_sigma0 * v_sigma0;
            let t98 = param_beta * t97;
            let t99 = t29 * t29;
            let t100 = t99 * t29;
            let t102 = f64x8::splat(1.0) / t30 / t100;
            let t104 = t38 * param_omega;
            let t105 = f64x8::splat(1.0) / t36;
            let t107 = t104 * param_gamma * t105;
            let t110 = f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t102 * t107 - f64x8::splat(8.0) / f64x8::splat(3.0) * t28 * t94 * t38;
            let t115 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t82 * t41 - t91 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t110));
            let t116 = t47 * t75;
            let t118 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t116)));
            let t121 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t118));
            let t122 = t121 * t26;
            let t126 = t54 * t87;
            let t129 = t5 * t126 * t69 / f64x8::splat(8.0);
            let t131 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t69 - t129));
            let tvrho0 = t45 + t73 + t6 * (t115 + t131);
            acc_vrho_0 = tvrho0;
            let t135 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t76)));
            let t138 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t135));
            let t139 = t138 * t26;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t41 - t91));
            let t146 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t116)));
            let t149 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t146));
            let t150 = t149 * t26;
            let t154 = t57 * v_rho1;
            let t156 = f64x8::splat(1.0) / t59 / t154;
            let t159 = v_sigma2 * v_sigma2;
            let t160 = param_beta * t159;
            let t161 = t57 * t57;
            let t162 = t161 * t57;
            let t164 = f64x8::splat(1.0) / t58 / t162;
            let t166 = t66 * param_omega;
            let t167 = f64x8::splat(1.0) / t64;
            let t169 = t166 * param_gamma * t167;
            let t172 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t156 * t66 + f64x8::splat(8.0) / f64x8::splat(3.0) * t160 * t164 * t169;
            let t177 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t69 - t129 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t172));
            let tvrho1 = t45 + t73 + t6 * (t144 + t177);
            acc_vrho_1 = tvrho1;
            let t182 = t99 * v_rho0;
            let t184 = f64x8::splat(1.0) / t30 / t182;
            let t187 = -t28 * t184 * t107 + param_beta * t33 * t38;
            let t191 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t187));
            let tvsigma0 = t6 * t191;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t194 = t161 * v_rho1;
            let t196 = f64x8::splat(1.0) / t58 / t194;
            let t199 = -t56 * t196 * t169 + param_beta * t61 * t66;
            let t203 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t199));
            let tvsigma2 = t6 * t203;
            acc_vsigma_2 = tvsigma2;
            let t206 = t23 * t23;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t78 * t78;
            let t211 = t74 * t6;
            let t212 = f64x8::splat(1.0) / t211;
            let t213 = t16 * t212;
            let t216 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t213)));
            let t220 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t207 * t208 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t216));
            let t221 = t220 * t26;
            let t225 = t81 * t87;
            let t227 = t5 * t225 * t41;
            let t233 = f64x8::splat(1.0) / t86 / t6;
            let t234 = t25 * t233;
            let t237 = t5 * t234 * t41 / f64x8::splat(12.0);
            let t239 = t5 * t88 * t110;
            let t242 = f64x8::splat(1.0) / t31 / t99;
            let t246 = t99 * t92;
            let t248 = f64x8::splat(1.0) / t30 / t246;
            let t252 = t97 * v_sigma0;
            let t253 = param_beta * t252;
            let t254 = t99 * t99;
            let t255 = t254 * t29;
            let t256 = f64x8::splat(1.0) / t255;
            let t257 = t253 * t256;
            let t258 = param_omega * param_omega;
            let t259 = t38 * t258;
            let t260 = param_gamma * param_gamma;
            let t261 = t36 * t36;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t260 * t262;
            let t264 = t259 * t263;
            let t267 = t104 * t263;
            let t270 = f64x8::splat(88.0) / f64x8::splat(9.0) * t28 * t242 * t38 - f64x8::splat(24.0) * t98 * t248 * t107 + f64x8::splat(64.0) / f64x8::splat(9.0) * t257 * t264 + f64x8::splat(64.0) / f64x8::splat(9.0) * t257 * t267;
            let t275 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t221 * t41 - t227 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t82 * t110 + t237 - t239 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t270));
            let t276 = t52 * t52;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t118 * t118;
            let t281 = t47 * t212;
            let t284 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t281)));
            let t288 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t277 * t278 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t284));
            let t289 = t288 * t26;
            let t293 = t121 * t87;
            let t295 = t5 * t293 * t69;
            let t297 = t54 * t233;
            let t300 = t5 * t297 * t69 / f64x8::splat(12.0);
            let t302 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t289 * t69 - t295 / f64x8::splat(4.0) + t300));
            let tv2rho20 = f64x8::splat(2.0) * t115 + f64x8::splat(2.0) * t131 + t6 * (t275 + t302);
            acc_v2rho2_0 = tv2rho20;
            let t305 = t207 * t135;
            let t309 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t213)));
            let t313 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t305 * t78 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t309));
            let t314 = t313 * t26;
            let t318 = t138 * t87;
            let t320 = t5 * t318 * t41;
            let t328 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t314 * t41 - t320 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t110 - t227 / f64x8::splat(8.0) + t237 - t239 / f64x8::splat(8.0)));
            let t329 = t277 * t146;
            let t333 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t281)));
            let t337 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t329 * t118 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t333));
            let t338 = t337 * t26;
            let t342 = t149 * t87;
            let t344 = t5 * t342 * t69;
            let t351 = t5 * t126 * t172;
            let t354 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t338 * t69 - t344 / f64x8::splat(8.0) - t295 / f64x8::splat(8.0) + t300 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t172 - t351 / f64x8::splat(8.0)));
            let tv2rho21 = t115 + t131 + t144 + t177 + t6 * (t328 + t354);
            acc_v2rho2_1 = tv2rho21;
            let t359 = t135 * t135;
            let t364 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t213)));
            let t368 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t207 * t359 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t364));
            let t369 = t368 * t26;
            let t375 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t369 * t41 - t320 / f64x8::splat(4.0) + t237));
            let t376 = t146 * t146;
            let t381 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t75 + f64x8::splat(2.0) * t281)));
            let t385 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t277 * t376 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t381));
            let t386 = t385 * t26;
            let t396 = f64x8::splat(1.0) / t59 / t161;
            let t400 = t161 * t154;
            let t402 = f64x8::splat(1.0) / t58 / t400;
            let t406 = t159 * v_sigma2;
            let t407 = param_beta * t406;
            let t408 = t161 * t161;
            let t409 = t408 * t57;
            let t410 = f64x8::splat(1.0) / t409;
            let t411 = t407 * t410;
            let t412 = t66 * t258;
            let t413 = t64 * t64;
            let t414 = f64x8::splat(1.0) / t413;
            let t415 = t260 * t414;
            let t416 = t412 * t415;
            let t419 = t166 * t415;
            let t422 = f64x8::splat(88.0) / f64x8::splat(9.0) * t56 * t396 * t66 - f64x8::splat(24.0) * t160 * t402 * t169 + f64x8::splat(64.0) / f64x8::splat(9.0) * t411 * t416 + f64x8::splat(64.0) / f64x8::splat(9.0) * t411 * t419;
            let t427 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t386 * t69 - t344 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t150 * t172 + t300 - t351 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t422));
            let tv2rho22 = f64x8::splat(2.0) * t144 + f64x8::splat(2.0) * t177 + t6 * (t375 + t427);
            acc_v2rho2_2 = tv2rho22;
            let t435 = t5 * t88 * t187 / f64x8::splat(8.0);
            let t440 = param_beta * t102 * t38;
            let t441 = param_omega * param_gamma;
            let t443 = t441 * v_sigma0 * t105;
            let t446 = t254 * v_rho0;
            let t447 = f64x8::splat(1.0) / t446;
            let t448 = t98 * t447;
            let t453 = -f64x8::splat(8.0) / f64x8::splat(3.0) * param_beta * t94 * t38 + f64x8::splat(8.0) * t440 * t443 - f64x8::splat(8.0) / f64x8::splat(3.0) * t448 * t264 - f64x8::splat(8.0) / f64x8::splat(3.0) * t448 * t267;
            let t458 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t82 * t187 - t435 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t453));
            let tv2rhosigma0 = t6 * t458 + t191;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t465 = t5 * t126 * t199 / f64x8::splat(8.0);
            let t467 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t199 - t465));
            let tv2rhosigma2 = t6 * t467 + t203;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t473 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t187 - t435));
            let tv2rhosigma3 = t6 * t473 + t191;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t482 = param_beta * t164 * t66;
            let t484 = t441 * v_sigma2 * t167;
            let t487 = t408 * v_rho1;
            let t488 = f64x8::splat(1.0) / t487;
            let t489 = t160 * t488;
            let t494 = -f64x8::splat(8.0) / f64x8::splat(3.0) * param_beta * t156 * t66 + f64x8::splat(8.0) * t482 * t484 - f64x8::splat(8.0) / f64x8::splat(3.0) * t489 * t416 - f64x8::splat(8.0) / f64x8::splat(3.0) * t489 * t419;
            let t499 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t199 - t465 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t494));
            let tv2rhosigma5 = t6 * t499 + t203;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t503 = t441 * t105;
            let t506 = f64x8::splat(1.0) / t254;
            let t507 = t28 * t506;
            let t510 = -f64x8::splat(2.0) * param_beta * t184 * t38 * t503 + t507 * t264 + t507 * t267;
            let t514 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t510));
            let tv2sigma20 = t6 * t514;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t517 = t441 * t167;
            let t520 = f64x8::splat(1.0) / t408;
            let t521 = t56 * t520;
            let t524 = -f64x8::splat(2.0) * param_beta * t196 * t66 * t517 + t521 * t416 + t521 * t419;
            let t528 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t524));
            let tv2sigma25 = t6 * t528;
            acc_v2sigma2_5 = tv2sigma25;
            let t532 = f64x8::splat(1.0) / t206 / t19;
            let t533 = t208 * t78;
            let t536 = t207 * t78;
            let t539 = t74 * t74;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t16 * t540;
            let t544 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t212 - f64x8::splat(6.0) * t541)));
            let t548 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t532 * t533 + f64x8::splat(4.0) / f64x8::splat(3.0) * t536 * t216 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t544));
            let t549 = t548 * t26;
            let t553 = t220 * t87;
            let t555 = t5 * t553 * t41;
            let t560 = t81 * t233;
            let t562 = t5 * t560 * t41;
            let t565 = t5 * t225 * t110;
            let t571 = f64x8::splat(1.0) / t86 / t74;
            let t572 = t25 * t571;
            let t575 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t572 * t41;
            let t577 = t5 * t234 * t110;
            let t580 = t5 * t88 * t270;
            let t583 = f64x8::splat(1.0) / t31 / t182;
            let t588 = f64x8::splat(1.0) / t30 / t254;
            let t592 = t254 * t92;
            let t593 = f64x8::splat(1.0) / t592;
            let t594 = t253 * t593;
            let t599 = t97 * t97;
            let t600 = param_beta * t599;
            let t601 = t254 * t182;
            let t603 = f64x8::splat(1.0) / t31 / t601;
            let t604 = t600 * t603;
            let t605 = t258 * param_omega;
            let t606 = t38 * t605;
            let t607 = t260 * param_gamma;
            let t609 = f64x8::splat(1.0) / t261 / t36;
            let t610 = t607 * t609;
            let t611 = t606 * t610;
            let t614 = t259 * t610;
            let t617 = t104 * t610;
            let t620 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t28 * t583 * t38 + f64x8::splat(5456.0) / f64x8::splat(27.0) * t98 * t588 * t107 - f64x8::splat(1216.0) / f64x8::splat(9.0) * t594 * t264 - f64x8::splat(1216.0) / f64x8::splat(9.0) * t594 * t267 + f64x8::splat(512.0) / f64x8::splat(27.0) * t604 * t611 + f64x8::splat(512.0) / f64x8::splat(9.0) * t604 * t614 + f64x8::splat(1024.0) / f64x8::splat(27.0) * t604 * t617;
            let t625 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t549 * t41 - f64x8::splat(3.0) / f64x8::splat(8.0) * t555 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t221 * t110 + t562 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t565 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t82 * t270 - t575 + t577 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t580 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t620));
            let t627 = f64x8::splat(1.0) / t276 / t50;
            let t628 = t278 * t118;
            let t631 = t277 * t118;
            let t634 = t47 * t540;
            let t637 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t212 - f64x8::splat(6.0) * t634)));
            let t641 = ((t51).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t627 * t628 + f64x8::splat(4.0) / f64x8::splat(3.0) * t631 * t284 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t637));
            let t642 = t641 * t26;
            let t646 = t288 * t87;
            let t648 = t5 * t646 * t69;
            let t650 = t121 * t233;
            let t652 = t5 * t650 * t69;
            let t654 = t54 * t571;
            let t657 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t654 * t69;
            let t659 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t642 * t69 - f64x8::splat(3.0) / f64x8::splat(8.0) * t648 + t652 / f64x8::splat(4.0) - t657));
            let tv3rho30 = f64x8::splat(3.0) * t275 + f64x8::splat(3.0) * t302 + t6 * (t625 + t659);
            acc_v3rho3_0 = tv3rho30;
            let t662 = f64x8::splat(2.0) * t328;
            let t663 = f64x8::splat(2.0) * t354;
            let t664 = t532 * t135;
            let t667 = t207 * t309;
            let t672 = f64x8::splat(2.0) * t212;
            let t673 = f64x8::splat(6.0) * t541;
            let t675 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t672 - t673)));
            let t679 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t664 * t208 + f64x8::splat(8.0) / f64x8::splat(9.0) * t667 * t78 + f64x8::splat(4.0) / f64x8::splat(9.0) * t305 * t216 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t675));
            let t680 = t679 * t26;
            let t684 = t313 * t87;
            let t687 = t5 * t684 * t41 / f64x8::splat(4.0);
            let t691 = t138 * t233;
            let t693 = t5 * t691 * t41;
            let t697 = t5 * t318 * t110 / f64x8::splat(4.0);
            let t706 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t680 * t41 - t687 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t314 * t110 + t693 / f64x8::splat(12.0) - t697 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t270 - t555 / f64x8::splat(8.0) + t562 / f64x8::splat(6.0) - t565 / f64x8::splat(4.0) - t575 + t577 / f64x8::splat(6.0) - t580 / f64x8::splat(8.0);
            let t707 = ((t1).select(f64x8::splat(0.0), t706));
            let t708 = t627 * t146;
            let t711 = t277 * t333;
            let t716 = f64x8::splat(6.0) * t634;
            let t718 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t672 - t716)));
            let t722 = ((t51).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t708 * t278 + f64x8::splat(8.0) / f64x8::splat(9.0) * t711 * t118 + f64x8::splat(4.0) / f64x8::splat(9.0) * t329 * t284 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t718));
            let t723 = t722 * t26;
            let t727 = t337 * t87;
            let t730 = t5 * t727 * t69 / f64x8::splat(4.0);
            let t731 = t149 * t233;
            let t733 = t5 * t731 * t69;
            let t742 = t5 * t293 * t172 / f64x8::splat(4.0);
            let t744 = t5 * t297 * t172;
            let t747 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t723 * t69 - t730 + t733 / f64x8::splat(12.0) - t648 / f64x8::splat(8.0) + t652 / f64x8::splat(6.0) - t657 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t289 * t172 - t742 + t744 / f64x8::splat(12.0)));
            let tv3rho31 = t275 + t302 + t662 + t663 + t6 * (t707 + t747);
            acc_v3rho3_1 = tv3rho31;
            let t750 = t532 * t359;
            let t755 = t207 * t364;
            let t759 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t672 - t673)));
            let t763 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t750 * t78 + f64x8::splat(8.0) / f64x8::splat(9.0) * t305 * t309 + f64x8::splat(4.0) / f64x8::splat(9.0) * t755 * t78 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t759));
            let t764 = t763 * t26;
            let t768 = t368 * t87;
            let t770 = t5 * t768 * t41;
            let t779 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t764 * t41 - t770 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t369 * t110 - t687 + t693 / f64x8::splat(6.0) - t697 + t562 / f64x8::splat(12.0) - t575 + t577 / f64x8::splat(12.0)));
            let t780 = t627 * t376;
            let t785 = t277 * t381;
            let t789 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t672 - t716)));
            let t793 = ((t51).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t780 * t118 + f64x8::splat(8.0) / f64x8::splat(9.0) * t329 * t333 + f64x8::splat(4.0) / f64x8::splat(9.0) * t785 * t118 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t789));
            let t794 = t793 * t26;
            let t798 = t385 * t87;
            let t800 = t5 * t798 * t69;
            let t807 = t5 * t342 * t172;
            let t815 = t5 * t126 * t422;
            let t817 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t794 * t69 - t800 / f64x8::splat(8.0) - t730 + t733 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t338 * t172 - t807 / f64x8::splat(4.0) + t652 / f64x8::splat(12.0) - t657 - t742 + t744 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t422 - t815 / f64x8::splat(8.0);
            let t818 = ((t46).select(f64x8::splat(0.0), t817));
            let tv3rho32 = t662 + t663 + t375 + t427 + t6 * (t779 + t818);
            acc_v3rho3_2 = tv3rho32;
            let t823 = t359 * t135;
            let t830 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t212 - f64x8::splat(6.0) * t541)));
            let t834 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t532 * t823 + f64x8::splat(4.0) / f64x8::splat(3.0) * t305 * t364 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t830));
            let t835 = t834 * t26;
            let t842 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t835 * t41 - f64x8::splat(3.0) / f64x8::splat(8.0) * t770 + t693 / f64x8::splat(4.0) - t575));
            let t843 = t376 * t146;
            let t850 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t212 - f64x8::splat(6.0) * t634)));
            let t854 = ((t51).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t627 * t843 + f64x8::splat(4.0) / f64x8::splat(3.0) * t329 * t381 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t850));
            let t855 = t854 * t26;
            let t871 = f64x8::splat(1.0) / t59 / t194;
            let t876 = f64x8::splat(1.0) / t58 / t408;
            let t880 = t408 * t154;
            let t881 = f64x8::splat(1.0) / t880;
            let t882 = t407 * t881;
            let t887 = t159 * t159;
            let t888 = param_beta * t887;
            let t889 = t408 * t194;
            let t891 = f64x8::splat(1.0) / t59 / t889;
            let t892 = t888 * t891;
            let t893 = t66 * t605;
            let t895 = f64x8::splat(1.0) / t413 / t64;
            let t896 = t607 * t895;
            let t897 = t893 * t896;
            let t900 = t412 * t896;
            let t903 = t166 * t896;
            let t906 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t56 * t871 * t66 + f64x8::splat(5456.0) / f64x8::splat(27.0) * t160 * t876 * t169 - f64x8::splat(1216.0) / f64x8::splat(9.0) * t882 * t416 - f64x8::splat(1216.0) / f64x8::splat(9.0) * t882 * t419 + f64x8::splat(512.0) / f64x8::splat(27.0) * t892 * t897 + f64x8::splat(512.0) / f64x8::splat(9.0) * t892 * t900 + f64x8::splat(1024.0) / f64x8::splat(27.0) * t892 * t903;
            let t911 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t855 * t69 - f64x8::splat(3.0) / f64x8::splat(8.0) * t800 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t386 * t172 + t733 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t807 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t150 * t422 - t657 + t744 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t815 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t906));
            let tv3rho33 = f64x8::splat(3.0) * t375 + f64x8::splat(3.0) * t427 + t6 * (t842 + t911);
            acc_v3rho3_3 = tv3rho33;
            let t919 = t5 * t225 * t187;
            let t926 = t5 * t234 * t187 / f64x8::splat(12.0);
            let t928 = t5 * t88 * t453;
            let t934 = param_beta * t248 * t38;
            let t938 = param_beta * t256 * t38;
            let t939 = t258 * t260;
            let t940 = t97 * t262;
            let t941 = t939 * t940;
            let t944 = param_omega * t260;
            let t945 = t944 * t940;
            let t948 = t254 * t99;
            let t950 = f64x8::splat(1.0) / t31 / t948;
            let t951 = t253 * t950;
            let t958 = f64x8::splat(88.0) / f64x8::splat(9.0) * param_beta * t242 * t38 - f64x8::splat(520.0) / f64x8::splat(9.0) * t934 * t443 + f64x8::splat(136.0) / f64x8::splat(3.0) * t938 * t941 + f64x8::splat(136.0) / f64x8::splat(3.0) * t938 * t945 - f64x8::splat(64.0) / f64x8::splat(9.0) * t951 * t611 - f64x8::splat(64.0) / f64x8::splat(3.0) * t951 * t614 - f64x8::splat(128.0) / f64x8::splat(9.0) * t951 * t617;
            let t963 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t221 * t187 - t919 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t82 * t453 + t926 - t928 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t958));
            let tv3rho2sigma0 = t6 * t963 + f64x8::splat(2.0) * t458;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t970 = t5 * t293 * t199;
            let t974 = t5 * t297 * t199 / f64x8::splat(12.0);
            let t976 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t289 * t199 - t970 / f64x8::splat(4.0) + t974));
            let tv3rho2sigma2 = t6 * t976 + f64x8::splat(2.0) * t467;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t982 = t5 * t318 * t187;
            let t990 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t314 * t187 - t982 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t453 - t919 / f64x8::splat(8.0) + t926 - t928 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t990 + t458 + t473;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t996 = t5 * t342 * t199;
            let t1003 = t5 * t126 * t494;
            let t1006 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t338 * t199 - t996 / f64x8::splat(8.0) - t970 / f64x8::splat(8.0) + t974 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t494 - t1003 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1006 + t467 + t499;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1014 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t369 * t187 - t982 / f64x8::splat(4.0) + t926));
            let tv3rho2sigma6 = t6 * t1014 + f64x8::splat(2.0) * t473;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1029 = param_beta * t402 * t66;
            let t1033 = param_beta * t410 * t66;
            let t1034 = t159 * t414;
            let t1035 = t939 * t1034;
            let t1038 = t944 * t1034;
            let t1041 = t408 * t161;
            let t1043 = f64x8::splat(1.0) / t59 / t1041;
            let t1044 = t407 * t1043;
            let t1051 = f64x8::splat(88.0) / f64x8::splat(9.0) * param_beta * t396 * t66 - f64x8::splat(520.0) / f64x8::splat(9.0) * t1029 * t484 + f64x8::splat(136.0) / f64x8::splat(3.0) * t1033 * t1035 + f64x8::splat(136.0) / f64x8::splat(3.0) * t1033 * t1038 - f64x8::splat(64.0) / f64x8::splat(9.0) * t1044 * t897 - f64x8::splat(64.0) / f64x8::splat(3.0) * t1044 * t900 - f64x8::splat(128.0) / f64x8::splat(9.0) * t1044 * t903;
            let t1056 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t386 * t199 - t996 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t150 * t494 + t974 - t1003 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t1051));
            let tv3rho2sigma8 = t6 * t1056 + f64x8::splat(2.0) * t499;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1063 = t5 * t88 * t510 / f64x8::splat(8.0);
            let t1067 = param_beta * t447 * t38;
            let t1068 = t262 * v_sigma0;
            let t1069 = t939 * t1068;
            let t1072 = t944 * t1068;
            let t1076 = f64x8::splat(1.0) / t31 / t592;
            let t1077 = t98 * t1076;
            let t1084 = f64x8::splat(32.0) / f64x8::splat(3.0) * t440 * t503 - f64x8::splat(40.0) / f64x8::splat(3.0) * t1067 * t1069 - f64x8::splat(40.0) / f64x8::splat(3.0) * t1067 * t1072 + f64x8::splat(8.0) / f64x8::splat(3.0) * t1077 * t611 + f64x8::splat(8.0) * t1077 * t614 + f64x8::splat(16.0) / f64x8::splat(3.0) * t1077 * t617;
            let t1089 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t82 * t510 - t1063 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1084));
            let tv3rhosigma20 = t6 * t1089 + t514;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1096 = t5 * t126 * t524 / f64x8::splat(8.0);
            let t1098 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t524 - t1096));
            let tv3rhosigma25 = t6 * t1098 + t528;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1104 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t510 - t1063));
            let tv3rhosigma26 = t6 * t1104 + t514;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1112 = param_beta * t488 * t66;
            let t1113 = t414 * v_sigma2;
            let t1114 = t939 * t1113;
            let t1117 = t944 * t1113;
            let t1121 = f64x8::splat(1.0) / t59 / t880;
            let t1122 = t160 * t1121;
            let t1129 = f64x8::splat(32.0) / f64x8::splat(3.0) * t482 * t517 - f64x8::splat(40.0) / f64x8::splat(3.0) * t1112 * t1114 - f64x8::splat(40.0) / f64x8::splat(3.0) * t1112 * t1117 + f64x8::splat(8.0) / f64x8::splat(3.0) * t1122 * t897 + f64x8::splat(8.0) * t1122 * t900 + f64x8::splat(16.0) / f64x8::splat(3.0) * t1122 * t903;
            let t1134 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t524 - t1096 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t1129));
            let tv3rhosigma211 = t6 * t1134 + t528;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1137 = param_beta * t506 * t38;
            let t1138 = t939 * t262;
            let t1141 = t944 * t262;
            let t1145 = f64x8::splat(1.0) / t31 / t255;
            let t1146 = t28 * t1145;
            let t1152 = f64x8::splat(3.0) * t1137 * t1138 + f64x8::splat(3.0) * t1137 * t1141 - t1146 * t611 - f64x8::splat(3.0) * t1146 * t614 - f64x8::splat(2.0) * t1146 * t617;
            let t1156 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1152));
            let tv3sigma30 = t6 * t1156;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1158 = param_beta * t520 * t66;
            let t1159 = t939 * t414;
            let t1162 = t944 * t414;
            let t1166 = f64x8::splat(1.0) / t59 / t409;
            let t1167 = t56 * t1166;
            let t1173 = f64x8::splat(3.0) * t1158 * t1159 + f64x8::splat(3.0) * t1158 * t1162 - t1167 * t897 - f64x8::splat(3.0) * t1167 * t900 - f64x8::splat(2.0) * t1167 * t903;
            let t1177 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t1173));
            let tv3sigma39 = t6 * t1177;
            acc_v3sigma3_9 = tv3sigma39;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
