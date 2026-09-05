//! LDA_C_PZ kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pz.c`
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
pub fn lda_c_pz_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_gamma_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_a_0: f64,
    param_c_0: f64,
    param_d_0: f64,
    param_b_0: f64,
    param_gamma_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_a_1: f64,
    param_c_1: f64,
    param_d_1: f64,
    param_b_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma_0 = f64x8::splat(param_gamma_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_a_0 = f64x8::splat(param_a_0);
    let param_c_0 = f64x8::splat(param_c_0);
    let param_d_0 = f64x8::splat(param_d_0);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_gamma_1 = f64x8::splat(param_gamma_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_c_1 = f64x8::splat(param_c_1);
    let param_d_1 = f64x8::splat(param_d_1);
    let param_b_1 = f64x8::splat(param_b_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t1 * t3 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = (f64x8::splat(1.0)).simd_le(t12);
            let t14 = param_gamma_0;
            let t15 = param_beta1_0;
            let t16 = ((t11).sqrt());
            let t20 = param_beta2_0 * t1;
            let t21 = t3 * t6;
            let t22 = t21 * t9;
            let t25 = f64x8::splat(1.0) + t15 * t16 / f64x8::splat(2.0) + t20 * t22 / f64x8::splat(4.0);
            let t28 = param_a_0;
            let t29 = (simd::ln(t12));
            let t33 = param_c_0 * t1;
            let t34 = t33 * t3;
            let t35 = t10 * t29;
            let t39 = param_d_0 * t1;
            let t43 = ((t13).select(t14 / t25, t28 * t29 + param_b_0 + t34 * t35 / f64x8::splat(4.0) + t39 * t22 / f64x8::splat(4.0)));
            let t44 = param_gamma_1;
            let t45 = param_beta1_1;
            let t49 = param_beta2_1 * t1;
            let t52 = f64x8::splat(1.0) + t45 * t16 / f64x8::splat(2.0) + t49 * t22 / f64x8::splat(4.0);
            let t55 = param_a_1;
            let t59 = param_c_1 * t1;
            let t60 = t59 * t3;
            let t64 = param_d_1 * t1;
            let t68 = ((t13).select(t44 / t52, t55 * t29 + param_b_1 + t60 * t35 / f64x8::splat(4.0) + t64 * t22 / f64x8::splat(4.0)));
            let t69 = t68 - t43;
            let t70 = v_rho0 - v_rho1;
            let t71 = f64x8::splat(1.0) / t7;
            let t72 = t70 * t71;
            let t73 = f64x8::splat(1.0) + t72;
            let t74 = (t73).simd_le(zeta_threshold);
            let t75 = (simd::cbrt(zeta_threshold));
            let t76 = t75 * zeta_threshold;
            let t77 = (simd::cbrt(t73));
            let t79 = ((t74).select(t76, t77 * t73));
            let t80 = f64x8::splat(1.0) - t72;
            let t81 = (t80).simd_le(zeta_threshold);
            let t82 = (simd::cbrt(t80));
            let t84 = ((t81).select(t76, t82 * t80));
            let t85 = t79 + t84 - f64x8::splat(2.0);
            let t87 = f64x8::splat(M_CBRT2);
            let t90 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t87 - f64x8::splat(2.0));
            let t91 = t69 * t85 * t90;
            let tzk0 = t43 + t91;
            acc_zk = tzk0;
            let t92 = t25 * t25;
            let t94 = t14 / t92;
            let t95 = f64x8::splat(1.0) / t16;
            let t97 = t15 * t95 * t1;
            let t99 = f64x8::splat(1.0) / t8 / t7;
            let t100 = t21 * t99;
            let t104 = -t20 * t100 / f64x8::splat(12.0) - t97 * t100 / f64x8::splat(12.0);
            let t109 = t6 * t99 * t29;
            let t117 = ((t13).select(-t94 * t104, -t28 * t71 / f64x8::splat(3.0) - t34 * t109 / f64x8::splat(12.0) - t33 * t100 / f64x8::splat(12.0) - t39 * t100 / f64x8::splat(12.0)));
            let t118 = t52 * t52;
            let t120 = t44 / t118;
            let t122 = t45 * t95 * t1;
            let t126 = -t122 * t100 / f64x8::splat(12.0) - t49 * t100 / f64x8::splat(12.0);
            let t137 = ((t13).select(-t120 * t126, -t55 * t71 / f64x8::splat(3.0) - t60 * t109 / f64x8::splat(12.0) - t59 * t100 / f64x8::splat(12.0) - t64 * t100 / f64x8::splat(12.0)));
            let t138 = t137 - t117;
            let t140 = t138 * t85 * t90;
            let t141 = t7 * t7;
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t70 * t142;
            let t144 = t71 - t143;
            let t147 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t144));
            let t148 = -t144;
            let t151 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t148));
            let t152 = t147 + t151;
            let t154 = t69 * t152 * t90;
            let tvrho0 = t43 + t91 + t7 * (t117 + t140 + t154);
            acc_vrho_0 = tvrho0;
            let t157 = -t71 - t143;
            let t160 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t157));
            let t161 = -t157;
            let t164 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t161));
            let t165 = t160 + t164;
            let t167 = t69 * t165 * t90;
            let tvrho1 = t43 + t91 + t7 * (t117 + t140 + t167);
            acc_vrho_1 = tvrho1;
            let t170 = f64x8::splat(2.0) * t117;
            let t171 = f64x8::splat(2.0) * t140;
            let t175 = t14 / t92 / t25;
            let t176 = t104 * t104;
            let t180 = f64x8::splat(1.0) / t16 / t11;
            let t182 = t1 * t1;
            let t183 = t15 * t180 * t182;
            let t184 = t3 * t3;
            let t185 = t184 * t5;
            let t186 = t8 * t8;
            let t189 = t185 / t186 / t141;
            let t193 = f64x8::splat(1.0) / t8 / t141;
            let t194 = t21 * t193;
            let t199 = -t183 * t189 / f64x8::splat(18.0) + t97 * t194 / f64x8::splat(9.0) + t20 * t194 / f64x8::splat(9.0);
            let t205 = t6 * t193 * t29;
            let t213 = ((t13).select(f64x8::splat(2.0) * t175 * t176 - t94 * t199, t28 * t142 / f64x8::splat(3.0) + t34 * t205 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(36.0) * t33 * t194 + t39 * t194 / f64x8::splat(9.0)));
            let t216 = t44 / t118 / t52;
            let t217 = t126 * t126;
            let t221 = t45 * t180 * t182;
            let t228 = -t221 * t189 / f64x8::splat(18.0) + t122 * t194 / f64x8::splat(9.0) + t49 * t194 / f64x8::splat(9.0);
            let t240 = ((t13).select(-t120 * t228 + f64x8::splat(2.0) * t216 * t217, t55 * t142 / f64x8::splat(3.0) + t60 * t205 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(36.0) * t59 * t194 + t64 * t194 / f64x8::splat(9.0)));
            let t241 = t240 - t213;
            let t243 = t241 * t85 * t90;
            let t245 = t138 * t152 * t90;
            let t246 = f64x8::splat(2.0) * t245;
            let t247 = t77 * t77;
            let t248 = f64x8::splat(1.0) / t247;
            let t249 = t144 * t144;
            let t252 = t141 * t7;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t70 * t253;
            let t256 = -f64x8::splat(2.0) * t142 + f64x8::splat(2.0) * t254;
            let t260 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t248 * t249 + f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t256));
            let t261 = t82 * t82;
            let t262 = f64x8::splat(1.0) / t261;
            let t263 = t148 * t148;
            let t266 = -t256;
            let t270 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t262 * t263 + f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t266));
            let t271 = t260 + t270;
            let t273 = t69 * t271 * t90;
            let tv2rho20 = t170 + t171 + f64x8::splat(2.0) * t154 + t7 * (t213 + t243 + t246 + t273);
            acc_v2rho2_0 = tv2rho20;
            let t277 = t138 * t165 * t90;
            let t278 = t248 * t157;
            let t281 = t77 * t70;
            let t285 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t278 * t144 + f64x8::splat(8.0) / f64x8::splat(3.0) * t281 * t253));
            let t286 = t262 * t161;
            let t289 = t82 * t70;
            let t293 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t286 * t148 - f64x8::splat(8.0) / f64x8::splat(3.0) * t289 * t253));
            let t294 = t285 + t293;
            let t296 = t69 * t294 * t90;
            let tv2rho21 = t170 + t171 + t154 + t167 + t7 * (t213 + t243 + t245 + t277 + t296);
            acc_v2rho2_1 = tv2rho21;
            let t300 = f64x8::splat(2.0) * t277;
            let t301 = t157 * t157;
            let t305 = f64x8::splat(2.0) * t142 + f64x8::splat(2.0) * t254;
            let t309 = ((t74).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t248 * t301 + f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t305));
            let t310 = t161 * t161;
            let t313 = -t305;
            let t317 = ((t81).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t262 * t310 + f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t313));
            let t318 = t309 + t317;
            let t320 = t69 * t318 * t90;
            let tv2rho22 = t170 + t171 + f64x8::splat(2.0) * t167 + t7 * (t213 + t243 + t300 + t320);
            acc_v2rho2_2 = tv2rho22;
            let t323 = f64x8::splat(3.0) * t213;
            let t324 = f64x8::splat(3.0) * t243;
            let t327 = t92 * t92;
            let t329 = t14 / t327;
            let t342 = f64x8::splat(1.0) / t16 / t182 / t184 / t5 * t186 / f64x8::splat(4.0);
            let t343 = t15 * t342;
            let t344 = t141 * t141;
            let t345 = f64x8::splat(1.0) / t344;
            let t346 = t2 * t345;
            let t351 = t185 / t186 / t252;
            let t355 = f64x8::splat(1.0) / t8 / t252;
            let t356 = t21 * t355;
            let t361 = -t343 * t346 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t183 * t351 - f64x8::splat(7.0) / f64x8::splat(27.0) * t97 * t356 - f64x8::splat(7.0) / f64x8::splat(27.0) * t20 * t356;
            let t367 = t6 * t355 * t29;
            let t375 = ((t13).select(f64x8::splat(6.0) * t175 * t104 * t199 - f64x8::splat(6.0) * t329 * t176 * t104 - t94 * t361, -f64x8::splat(2.0) / f64x8::splat(3.0) * t28 * t253 - f64x8::splat(7.0) / f64x8::splat(27.0) * t34 * t367 - f64x8::splat(13.0) / f64x8::splat(36.0) * t33 * t356 - f64x8::splat(7.0) / f64x8::splat(27.0) * t39 * t356));
            let t376 = t118 * t118;
            let t378 = t44 / t376;
            let t385 = t45 * t342;
            let t394 = -t385 * t346 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t221 * t351 - f64x8::splat(7.0) / f64x8::splat(27.0) * t122 * t356 - f64x8::splat(7.0) / f64x8::splat(27.0) * t49 * t356;
            let t406 = ((t13).select(f64x8::splat(6.0) * t216 * t126 * t228 - f64x8::splat(6.0) * t378 * t217 * t126 - t120 * t394, -f64x8::splat(2.0) / f64x8::splat(3.0) * t55 * t253 - f64x8::splat(7.0) / f64x8::splat(27.0) * t60 * t367 - f64x8::splat(13.0) / f64x8::splat(36.0) * t59 * t356 - f64x8::splat(7.0) / f64x8::splat(27.0) * t64 * t356));
            let t407 = t406 - t375;
            let t409 = t407 * t85 * t90;
            let t411 = t241 * t152 * t90;
            let t412 = f64x8::splat(3.0) * t411;
            let t414 = t138 * t271 * t90;
            let t417 = f64x8::splat(1.0) / t247 / t73;
            let t418 = t249 * t144;
            let t421 = t248 * t144;
            let t424 = t70 * t345;
            let t426 = f64x8::splat(6.0) * t253 - f64x8::splat(6.0) * t424;
            let t430 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t417 * t418 + f64x8::splat(4.0) / f64x8::splat(3.0) * t421 * t256 + f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t426));
            let t432 = f64x8::splat(1.0) / t261 / t80;
            let t433 = t263 * t148;
            let t436 = t262 * t148;
            let t439 = -t426;
            let t443 = ((t81).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t432 * t433 + f64x8::splat(4.0) / f64x8::splat(3.0) * t436 * t266 + f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t439));
            let t444 = t430 + t443;
            let t446 = t69 * t444 * t90;
            let tv3rho30 = t323 + t324 + f64x8::splat(6.0) * t245 + f64x8::splat(3.0) * t273 + t7 * (t375 + t409 + t412 + f64x8::splat(3.0) * t414 + t446);
            acc_v3rho3_0 = tv3rho30;
            let t450 = f64x8::splat(2.0) * t296;
            let t453 = t241 * t165 * t90;
            let t455 = t138 * t294 * t90;
            let t456 = f64x8::splat(2.0) * t455;
            let t457 = t417 * t157;
            let t460 = t248 * t70;
            let t471 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t457 * t249 + f64x8::splat(16.0) / f64x8::splat(9.0) * t460 * t253 * t144 + f64x8::splat(4.0) / f64x8::splat(9.0) * t278 * t256 + f64x8::splat(8.0) / f64x8::splat(3.0) * t77 * t253 - f64x8::splat(8.0) * t281 * t345));
            let t472 = t432 * t161;
            let t475 = t262 * t70;
            let t486 = ((t81).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t472 * t263 - f64x8::splat(16.0) / f64x8::splat(9.0) * t475 * t253 * t148 + f64x8::splat(4.0) / f64x8::splat(9.0) * t286 * t266 - f64x8::splat(8.0) / f64x8::splat(3.0) * t82 * t253 + f64x8::splat(8.0) * t289 * t345));
            let t487 = t471 + t486;
            let t489 = t69 * t487 * t90;
            let tv3rho31 = t323 + t324 + f64x8::splat(4.0) * t245 + t273 + t300 + t450 + t7 * (t375 + t409 + f64x8::splat(2.0) * t411 + t414 + t453 + t456 + t489);
            acc_v3rho3_1 = tv3rho31;
            let t495 = t138 * t318 * t90;
            let t496 = t417 * t301;
            let t501 = t248 * t305;
            let t506 = -f64x8::splat(2.0) * t253 - f64x8::splat(6.0) * t424;
            let t510 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t496 * t144 + f64x8::splat(16.0) / f64x8::splat(9.0) * t278 * t254 + f64x8::splat(4.0) / f64x8::splat(9.0) * t501 * t144 + f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t506));
            let t511 = t432 * t310;
            let t516 = t262 * t313;
            let t519 = -t506;
            let t523 = ((t81).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t511 * t148 - f64x8::splat(16.0) / f64x8::splat(9.0) * t286 * t254 + f64x8::splat(4.0) / f64x8::splat(9.0) * t516 * t148 + f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t519));
            let t524 = t510 + t523;
            let t526 = t69 * t524 * t90;
            let tv3rho32 = t323 + t324 + t246 + f64x8::splat(4.0) * t277 + t450 + t320 + t7 * (t375 + t409 + t411 + f64x8::splat(2.0) * t453 + t456 + t495 + t526);
            acc_v3rho3_2 = tv3rho32;
            let t531 = f64x8::splat(3.0) * t453;
            let t533 = t301 * t157;
            let t539 = -f64x8::splat(6.0) * t253 - f64x8::splat(6.0) * t424;
            let t543 = ((t74).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t417 * t533 + f64x8::splat(4.0) / f64x8::splat(3.0) * t278 * t305 + f64x8::splat(4.0) / f64x8::splat(3.0) * t77 * t539));
            let t544 = t310 * t161;
            let t549 = -t539;
            let t553 = ((t81).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t432 * t544 + f64x8::splat(4.0) / f64x8::splat(3.0) * t286 * t313 + f64x8::splat(4.0) / f64x8::splat(3.0) * t82 * t549));
            let t554 = t543 + t553;
            let t556 = t69 * t554 * t90;
            let tv3rho33 = t323 + t324 + f64x8::splat(6.0) * t277 + f64x8::splat(3.0) * t320 + t7 * (t375 + t409 + t531 + f64x8::splat(3.0) * t495 + t556);
            acc_v3rho3_3 = tv3rho33;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}
