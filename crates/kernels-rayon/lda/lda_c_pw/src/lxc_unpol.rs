//! LDA_C_PW lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_pw_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_alpha1_0 = f64x8::splat(param_alpha1_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_beta3_0 = f64x8::splat(param_beta3_0);
    let param_pp_0 = f64x8::splat(param_pp_0);
    let param_beta4_0 = f64x8::splat(param_beta4_0);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_alpha1_2 = f64x8::splat(param_alpha1_2);
    let param_beta1_2 = f64x8::splat(param_beta1_2);
    let param_beta2_2 = f64x8::splat(param_beta2_2);
    let param_beta3_2 = f64x8::splat(param_beta3_2);
    let param_pp_2 = f64x8::splat(param_pp_2);
    let param_beta4_2 = f64x8::splat(param_beta4_2);
    let param_fz20 = f64x8::splat(param_fz20);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_alpha1_1 = f64x8::splat(param_alpha1_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_beta3_1 = f64x8::splat(param_beta3_1);
    let param_pp_1 = f64x8::splat(param_pp_1);
    let param_beta4_1 = f64x8::splat(param_beta4_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        {
            let t1 = param_a_0;
            let t2 = param_alpha1_0;
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t2 * t3;
            let t5 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t6 = (simd::cbrt(t5));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t7 * t7;
            let t9 = t6 * t8;
            let t10 = (simd::cbrt(v_rho));
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t15 = f64x8::splat(1.0) + t4 * t12 / f64x8::splat(4.0);
            let t17 = f64x8::splat(1.0) / t1;
            let t18 = param_beta1_0;
            let t19 = t3 * t6;
            let t21 = t19 * t8 * t11;
            let t22 = ((t21).sqrt());
            let t26 = param_beta2_0 * t3;
            let t29 = param_beta3_0;
            let t30 = ((t21) * (t21).sqrt());
            let t34 = t21 / f64x8::splat(4.0);
            let t36 = param_pp_0 + f64x8::splat(1.0);
            let t37 = (simd::pow(t34, t36));
            let t38 = param_beta4_0 * t37;
            let t39 = t18 * t22 / f64x8::splat(2.0) + t26 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t29 * t30 + t38;
            let t43 = f64x8::splat(1.0) + t17 / t39 / f64x8::splat(2.0);
            let t44 = (simd::ln(t43));
            let t45 = t1 * t15 * t44;
            let t47 = (simd::cbrt(zeta_threshold));
            let t49 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t47 * zeta_threshold, f64x8::splat(1.0)));
            let t52 = f64x8::splat(M_CBRT2);
            let t56 = (f64x8::splat(2.0) * t49 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t52 - f64x8::splat(2.0));
            let t57 = param_a_2;
            let t59 = param_alpha1_2;
            let t60 = t59 * t3;
            let t63 = f64x8::splat(1.0) + t60 * t12 / f64x8::splat(4.0);
            let t64 = f64x8::splat(1.0) / t57;
            let t65 = param_beta1_2;
            let t69 = param_beta2_2 * t3;
            let t72 = param_beta3_2;
            let t77 = param_pp_2 + f64x8::splat(1.0);
            let t78 = (simd::pow(t34, t77));
            let t79 = param_beta4_2 * t78;
            let t80 = t65 * t22 / f64x8::splat(2.0) + t69 * t12 / f64x8::splat(4.0) + f64x8::splat(0.125) * t72 * t30 + t79;
            let t84 = f64x8::splat(1.0) + t64 / t80 / f64x8::splat(2.0);
            let t85 = (simd::ln(t84));
            let t87 = f64x8::splat(1.0) / param_fz20;
            let t89 = t56 * t57 * t63 * t85 * t87;
            let tzk0 = -f64x8::splat(2.0) * t45 + f64x8::splat(2.0) * t89;
            acc_zk = tzk0;
            let t94 = t1 * t2 * t3;
            let t96 = f64x8::splat(1.0) / t10 / v_rho;
            let t99 = t94 * t9 * t96 * t44;
            let t101 = t39 * t39;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t15 * t102;
            let t104 = f64x8::splat(1.0) / t22;
            let t106 = t18 * t104 * t3;
            let t107 = t9 * t96;
            let t112 = ((t21).sqrt());
            let t114 = t29 * t112 * t3;
            let t117 = f64x8::splat(1.0) / v_rho;
            let t121 = -t106 * t107 / f64x8::splat(12.0) - t26 * t107 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t114 * t107 - t38 * t36 * t117 / f64x8::splat(3.0);
            let t122 = f64x8::splat(1.0) / t43;
            let t123 = t121 * t122;
            let t124 = t103 * t123;
            let t127 = t56 * t57 * t59 * t3;
            let t131 = t127 * t9 * t96 * t85 * t87;
            let t133 = t56 * t63;
            let t134 = t80 * t80;
            let t135 = f64x8::splat(1.0) / t134;
            let t137 = t65 * t104 * t3;
            let t143 = t72 * t112 * t3;
            let t149 = -t137 * t107 / f64x8::splat(12.0) - t69 * t107 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t143 * t107 - t79 * t77 * t117 / f64x8::splat(3.0);
            let t151 = f64x8::splat(1.0) / t84;
            let t152 = t151 * t87;
            let t154 = t133 * t135 * t149 * t152;
            let tvrho0 = -f64x8::splat(2.0) * t45 + f64x8::splat(2.0) * t89 + v_rho * (t99 / f64x8::splat(6.0) + t124 - t131 / f64x8::splat(6.0) - t154);
            acc_vrho = tvrho0;
            let t161 = v_rho * v_rho;
            let t163 = f64x8::splat(1.0) / t10 / t161;
            let t166 = t94 * t9 * t163 * t44;
            let t168 = t4 * t9;
            let t169 = t96 * t102;
            let t171 = t168 * t169 * t123;
            let t173 = t101 * t39;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t15 * t174;
            let t176 = t121 * t121;
            let t177 = t176 * t122;
            let t178 = t175 * t177;
            let t181 = f64x8::splat(1.0) / t22 / t21;
            let t183 = t3 * t3;
            let t184 = t18 * t181 * t183;
            let t185 = t6 * t6;
            let t186 = t185 * t7;
            let t187 = t10 * t10;
            let t190 = t186 / t187 / t161;
            let t193 = t9 * t163;
            let t198 = f64x8::splat(1.0)/((t21).sqrt());
            let t200 = t29 * t198 * t183;
            let t205 = t36 * t36;
            let t206 = f64x8::splat(1.0) / t161;
            let t213 = -t184 * t190 / f64x8::splat(18.0) + t106 * t193 / f64x8::splat(9.0) + t26 * t193 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t200 * t190 + f64x8::splat(0.08333333333333333) * t114 * t193 + t38 * t205 * t206 / f64x8::splat(9.0) + t38 * t36 * t206 / f64x8::splat(3.0);
            let t214 = t213 * t122;
            let t215 = t103 * t214;
            let t216 = t101 * t101;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t15 * t217;
            let t219 = t43 * t43;
            let t220 = f64x8::splat(1.0) / t219;
            let t222 = t176 * t220 * t17;
            let t223 = t218 * t222;
            let t228 = t127 * t9 * t163 * t85 * t87;
            let t231 = t56 * t60 * t6;
            let t232 = t8 * t96;
            let t233 = t232 * t135;
            let t234 = t149 * t151;
            let t235 = t234 * t87;
            let t237 = t231 * t233 * t235;
            let t239 = t134 * t80;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t149 * t149;
            let t244 = t133 * t240 * t241 * t152;
            let t247 = t65 * t181 * t183;
            let t255 = t72 * t198 * t183;
            let t260 = t77 * t77;
            let t267 = -t247 * t190 / f64x8::splat(18.0) + t137 * t193 / f64x8::splat(9.0) + t69 * t193 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t255 * t190 + f64x8::splat(0.08333333333333333) * t143 * t193 + t79 * t260 * t206 / f64x8::splat(9.0) + t79 * t77 * t206 / f64x8::splat(3.0);
            let t270 = t133 * t135 * t267 * t152;
            let t271 = t134 * t134;
            let t272 = f64x8::splat(1.0) / t271;
            let t274 = t56 * t63 * t272;
            let t275 = t84 * t84;
            let t276 = f64x8::splat(1.0) / t275;
            let t277 = t241 * t276;
            let t278 = t87 * t64;
            let t280 = t274 * t277 * t278;
            let tv2rho20 = t99 / f64x8::splat(3.0) + f64x8::splat(2.0) * t124 - t131 / f64x8::splat(3.0) - f64x8::splat(2.0) * t154 + v_rho * (-f64x8::splat(2.0) / f64x8::splat(9.0) * t166 - t171 / f64x8::splat(6.0) - f64x8::splat(2.0) * t178 + t215 + t223 / f64x8::splat(2.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t228 + t237 / f64x8::splat(6.0) + f64x8::splat(2.0) * t244 - t270 - t280 / f64x8::splat(2.0));
            acc_v2rho2 = tv2rho20;
            let t294 = t96 * t174;
            let t296 = t168 * t294 * t177;
            let t304 = f64x8::splat(1.0) / t22 / t183 / t185 / t7 * t187 / f64x8::splat(4.0);
            let t305 = t18 * t304;
            let t306 = t161 * t161;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t5 * t307;
            let t311 = t161 * v_rho;
            let t314 = t186 / t187 / t311;
            let t318 = f64x8::splat(1.0) / t10 / t311;
            let t319 = t9 * t318;
            let t324 = f64x8::splat(1.0)/((t21) * (t21).sqrt());
            let t325 = t29 * t324;
            let t332 = t205 * t36;
            let t333 = f64x8::splat(1.0) / t311;
            let t343 = -t305 * t308 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t184 * t314 - f64x8::splat(7.0) / f64x8::splat(27.0) * t106 * t319 - f64x8::splat(7.0) / f64x8::splat(27.0) * t26 * t319 + f64x8::splat(0.08333333333333333) * t325 * t308 - f64x8::splat(0.16666666666666666) * t200 * t314 - f64x8::splat(0.19444444444444445) * t114 * t319 - t38 * t332 * t333 / f64x8::splat(27.0) - t38 * t205 * t333 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t38 * t36 * t333;
            let t344 = t343 * t122;
            let t345 = t103 * t344;
            let t348 = t220 * t17 * t121;
            let t349 = t218 * t213 * t348;
            let t351 = t176 * t121;
            let t352 = t351 * t122;
            let t353 = t218 * t352;
            let t355 = t123 * t213;
            let t356 = t175 * t355;
            let t359 = f64x8::splat(1.0) / t216 / t39;
            let t360 = t15 * t359;
            let t362 = t351 * t220 * t17;
            let t363 = t360 * t362;
            let t366 = f64x8::splat(1.0) / t216 / t101;
            let t367 = t15 * t366;
            let t369 = f64x8::splat(1.0) / t219 / t43;
            let t371 = t1 * t1;
            let t372 = f64x8::splat(1.0) / t371;
            let t373 = t351 * t369 * t372;
            let t374 = t367 * t373;
            let t376 = t65 * t304;
            let t385 = t72 * t324;
            let t392 = t260 * t77;
            let t402 = -t376 * t308 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t247 * t314 - f64x8::splat(7.0) / f64x8::splat(27.0) * t137 * t319 - f64x8::splat(7.0) / f64x8::splat(27.0) * t69 * t319 + f64x8::splat(0.08333333333333333) * t385 * t308 - f64x8::splat(0.16666666666666666) * t255 * t314 - f64x8::splat(0.19444444444444445) * t143 * t319 - t79 * t392 * t333 / f64x8::splat(27.0) - t79 * t260 * t333 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t79 * t77 * t333;
            let t405 = t133 * t135 * t402 * t152;
            let t408 = t94 * t9 * t318 * t44;
            let t410 = t163 * t102;
            let t412 = t168 * t410 * t123;
            let t415 = t168 * t169 * t214;
            let t417 = t296 / f64x8::splat(2.0) + t345 + f64x8::splat(3.0) / f64x8::splat(2.0) * t349 + f64x8::splat(6.0) * t353 - f64x8::splat(6.0) * t356 - f64x8::splat(3.0) * t363 + t374 / f64x8::splat(2.0) - t405 + f64x8::splat(14.0) / f64x8::splat(27.0) * t408 + t412 / f64x8::splat(3.0) - t415 / f64x8::splat(4.0);
            let t418 = t241 * t149;
            let t421 = t133 * t272 * t418 * t152;
            let t424 = t56 * t63 * t240;
            let t425 = t87 * t267;
            let t427 = t424 * t234 * t425;
            let t430 = f64x8::splat(1.0) / t271 / t80;
            let t432 = t56 * t63 * t430;
            let t435 = t432 * t418 * t276 * t278;
            let t438 = f64x8::splat(1.0) / t271 / t134;
            let t440 = t56 * t63 * t438;
            let t442 = f64x8::splat(1.0) / t275 / t84;
            let t444 = t57 * t57;
            let t445 = f64x8::splat(1.0) / t444;
            let t446 = t87 * t445;
            let t448 = t440 * t418 * t442 * t446;
            let t453 = t127 * t9 * t318 * t85 * t87;
            let t455 = t8 * t163;
            let t456 = t455 * t135;
            let t458 = t231 * t456 * t235;
            let t461 = t267 * t151 * t87;
            let t463 = t231 * t233 * t461;
            let t465 = t96 * t217;
            let t467 = t168 * t465 * t222;
            let t470 = t278 * t149;
            let t472 = t274 * t267 * t276 * t470;
            let t475 = t241 * t151;
            let t476 = t475 * t87;
            let t478 = t231 * t232 * t240 * t476;
            let t482 = t56 * t59 * t19 * t8;
            let t483 = t96 * t272;
            let t485 = t276 * t87;
            let t486 = t485 * t64;
            let t488 = t482 * t483 * t241 * t486;
            let t490 = -f64x8::splat(6.0) * t421 + f64x8::splat(6.0) * t427 + f64x8::splat(3.0) * t435 - t448 / f64x8::splat(2.0) - f64x8::splat(14.0) / f64x8::splat(27.0) * t453 - t458 / f64x8::splat(3.0) + t463 / f64x8::splat(4.0) - t467 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t472 - t478 / f64x8::splat(2.0) + t488 / f64x8::splat(8.0);
            let tv3rho30 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t166 - t171 / f64x8::splat(2.0) - f64x8::splat(6.0) * t178 + f64x8::splat(3.0) * t215 + f64x8::splat(3.0) / f64x8::splat(2.0) * t223 + f64x8::splat(2.0) / f64x8::splat(3.0) * t228 + t237 / f64x8::splat(2.0) + f64x8::splat(6.0) * t244 - f64x8::splat(3.0) * t270 - f64x8::splat(3.0) / f64x8::splat(2.0) * t280 + v_rho * (t417 + t490);
            acc_v3rho3 = tv3rho30;
            let t503 = f64x8::splat(2.0) * t296 + f64x8::splat(4.0) * t345 + f64x8::splat(6.0) * t349 + f64x8::splat(24.0) * t353 - f64x8::splat(24.0) * t356 - f64x8::splat(12.0) * t363 + f64x8::splat(2.0) * t374 - f64x8::splat(4.0) * t405 + f64x8::splat(56.0) / f64x8::splat(27.0) * t408 + f64x8::splat(4.0) / f64x8::splat(3.0) * t412 - t415;
            let t514 = t213 * t213;
            let t518 = t176 * t176;
            let t539 = t216 * t216;
            let t542 = t219 * t219;
            let t565 = -f64x8::splat(6.0) * t175 * t514 * t122 - f64x8::splat(24.0) * t360 * t518 * t122 - f64x8::splat(6.0) * t15 / t216 / t173 * t518 * t369 * t372 + f64x8::splat(36.0) * t218 * t177 * t213 + f64x8::splat(18.0) * t367 * t518 * t220 * t17 - f64x8::splat(8.0) * t175 * t344 * t121 + f64x8::splat(3.0) / f64x8::splat(4.0) * t15 / t539 * t518 / t542 / t371 / t1 + f64x8::splat(3.0) / f64x8::splat(2.0) * t218 * t514 * t220 * t17 + f64x8::splat(3.0) * t367 * t213 * t369 * t372 * t176 + f64x8::splat(2.0) * t218 * t343 * t348 - f64x8::splat(18.0) * t360 * t213 * t222;
            let t569 = f64x8::splat(1.0) / t22 / t5 / t117 / f64x8::splat(48.0);
            let t572 = t306 * v_rho;
            let t576 = f64x8::splat(1.0) / t10 / t572 * t3 * t9;
            let t580 = t5 / t572;
            let t585 = t186 / t187 / t306;
            let t589 = f64x8::splat(1.0) / t10 / t306;
            let t590 = t9 * t589;
            let t595 = (simd::pow(t21, -f64x8::splat(2.5)));
            let t606 = t205 * t205;
            let t619 = -f64x8::splat(5.0) / f64x8::splat(18.0) * t18 * t569 * t5 * t576 + f64x8::splat(8.0) / f64x8::splat(3.0) * t305 * t580 - f64x8::splat(80.0) / f64x8::splat(81.0) * t184 * t585 + f64x8::splat(70.0) / f64x8::splat(81.0) * t106 * t590 + f64x8::splat(70.0) / f64x8::splat(81.0) * t26 * t590 + f64x8::splat(0.041666666666666664) * t29 * t595 * t5 * t576 - f64x8::splat(0.6666666666666666) * t325 * t580 + f64x8::splat(0.7407407407407407) * t200 * t585 + f64x8::splat(0.6481481481481481) * t114 * t590 + t38 * t606 * t307 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t38 * t332 * t307 + f64x8::splat(11.0) / f64x8::splat(9.0) * t38 * t205 * t307 + f64x8::splat(2.0) * t38 * t36 * t307;
            let t622 = t271 * t271;
            let t626 = t241 * t241;
            let t627 = t275 * t275;
            let t661 = t402 * t151;
            let t673 = t267 * t267;
            let t678 = t103 * t619 * t122 - f64x8::splat(3.0) / f64x8::splat(4.0) * t56 * t63 / t622 * t626 / t627 * t87 / t444 / t57 + f64x8::splat(6.0) * t56 * t63 / t271 / t239 * t626 * t442 * t446 - f64x8::splat(36.0) * t274 * t475 * t425 - f64x8::splat(18.0) * t440 * t626 * t276 * t278 - f64x8::splat(28.0) / f64x8::splat(27.0) * t168 * t318 * t102 * t123 + f64x8::splat(2.0) / f64x8::splat(3.0) * t168 * t410 * t214 - t168 * t169 * t344 / f64x8::splat(3.0) + f64x8::splat(8.0) * t424 * t661 * t87 * t149 - f64x8::splat(4.0) / f64x8::splat(3.0) * t168 * t163 * t174 * t177 - f64x8::splat(2.0) * t168 * t465 * t352 - f64x8::splat(3.0) / f64x8::splat(2.0) * t274 * t673 * t276 * t278;
            let t702 = t260 * t260;
            let t715 = -f64x8::splat(5.0) / f64x8::splat(18.0) * t65 * t569 * t5 * t576 + f64x8::splat(8.0) / f64x8::splat(3.0) * t376 * t580 - f64x8::splat(80.0) / f64x8::splat(81.0) * t247 * t585 + f64x8::splat(70.0) / f64x8::splat(81.0) * t137 * t590 + f64x8::splat(70.0) / f64x8::splat(81.0) * t69 * t590 + f64x8::splat(0.041666666666666664) * t72 * t595 * t5 * t576 - f64x8::splat(0.6666666666666666) * t385 * t580 + f64x8::splat(0.7407407407407407) * t255 * t585 + f64x8::splat(0.6481481481481481) * t143 * t590 + t79 * t702 * t307 / f64x8::splat(81.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t79 * t392 * t307 + f64x8::splat(11.0) / f64x8::splat(9.0) * t79 * t260 * t307 + f64x8::splat(2.0) * t79 * t77 * t307;
            let t765 = -t133 * t135 * t715 * t152 + f64x8::splat(6.0) * t133 * t240 * t673 * t152 + f64x8::splat(24.0) * t133 * t430 * t626 * t152 - f64x8::splat(140.0) / f64x8::splat(81.0) * t94 * t9 * t589 * t44 + t482 * t483 * t267 * t485 * t64 * t149 / f64x8::splat(2.0) - t4 * t107 * t217 * t213 * t348 / f64x8::splat(2.0) + f64x8::splat(140.0) / f64x8::splat(81.0) * t127 * t9 * t589 * t85 * t87 + t231 * t233 * t661 * t87 / f64x8::splat(3.0) + f64x8::splat(2.0) * t231 * t232 * t272 * t418 * t151 * t87 + f64x8::splat(28.0) / f64x8::splat(27.0) * t231 * t8 * t318 * t135 * t235 - f64x8::splat(2.0) / f64x8::splat(3.0) * t231 * t456 * t461;
            let t818 = f64x8::splat(4.0) / f64x8::splat(3.0) * t231 * t455 * t240 * t476 + f64x8::splat(2.0) * t168 * t294 * t355 + t168 * t96 * t359 * t362 - t168 * t96 * t366 * t373 / f64x8::splat(6.0) - f64x8::splat(2.0) * t274 * t402 * t276 * t470 + t168 * t163 * t217 * t222 / f64x8::splat(3.0) + f64x8::splat(18.0) * t432 * t277 * t425 * t64 - f64x8::splat(3.0) * t440 * t241 * t442 * t446 * t267 - f64x8::splat(2.0) * t482 * t96 * t240 * t149 * t461 - t482 * t96 * t430 * t418 * t486 + t482 * t96 * t438 * t418 * t442 * t87 * t445 / f64x8::splat(6.0) - t482 * t163 * t272 * t241 * t486 / f64x8::splat(3.0);
            let t822 = -f64x8::splat(24.0) * t421 + f64x8::splat(24.0) * t427 + f64x8::splat(12.0) * t435 - f64x8::splat(2.0) * t448 - f64x8::splat(56.0) / f64x8::splat(27.0) * t453 - f64x8::splat(4.0) / f64x8::splat(3.0) * t458 + t463 - t467 / f64x8::splat(2.0) - f64x8::splat(6.0) * t472 - f64x8::splat(2.0) * t478 + t488 / f64x8::splat(2.0) + v_rho * (t565 + t678 + t765 + t818);
            let tv4rho40 = t503 + t822;
            acc_v4rho4 = tv4rho40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
