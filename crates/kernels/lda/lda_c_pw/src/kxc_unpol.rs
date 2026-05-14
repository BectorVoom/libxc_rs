//! LDA_C_PW kxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_C_PW kxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_pw_kxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_a_0: f64,
    param_a_2: f64,
    param_alpha1_0: f64,
    param_alpha1_2: f64,
    param_beta1_0: f64,
    param_beta1_2: f64,
    param_beta2_0: f64,
    param_beta2_2: f64,
    param_beta3_0: f64,
    param_beta3_2: f64,
    param_beta4_0: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_pp_0: f64,
    param_pp_2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = param_a_0;
        let t2 = param_alpha1_0;
        let t3 = M_CBRT3;
        let t4 = t2 * t3;
        let t5 = 1.0 / M_PI;
        let t6 = pow_1_3(t5);
        let t7 = M_CBRT4;
        let t8 = t7 * t7;
        let t9 = t6 * t8;
        let t10 = pow_1_3(rho[ip]);
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t15 = 1.0 + t4 * t12 / 4.0;
        let t17 = 1.0 / t1;
        let t18 = param_beta1_0;
        let t19 = t3 * t6;
        let t21 = t19 * t8 * t11;
        let t22 = f64::sqrt(t21);
        let t26 = param_beta2_0 * t3;
        let t29 = param_beta3_0;
        let t30 = pow_3_2(t21);
        let t34 = t21 / 4.0;
        let t36 = param_pp_0 + 1.0;
        let t37 = f64::powf(t34, t36);
        let t38 = param_beta4_0 * t37;
        let t39 = t18 * t22 / 2.0 + t26 * t12 / 4.0 + 0.125 * t29 * t30 + t38;
        let t43 = 1.0 + t17 / t39 / 2.0;
        let t44 = f64::ln(t43);
        let t45 = t1 * t15 * t44;
        let t47 = pow_1_3(zeta_threshold);
        let t49 = piecewise3(1.0 <= zeta_threshold, t47 * zeta_threshold, 1.0);
        let t52 = M_CBRT2;
        let t56 = (2.0 * t49 - 2.0) / (2.0 * t52 - 2.0);
        let t57 = param_a_2;
        let t59 = param_alpha1_2;
        let t60 = t59 * t3;
        let t63 = 1.0 + t60 * t12 / 4.0;
        let t64 = 1.0 / t57;
        let t65 = param_beta1_2;
        let t69 = param_beta2_2 * t3;
        let t72 = param_beta3_2;
        let t77 = param_pp_2 + 1.0;
        let t78 = f64::powf(t34, t77);
        let t79 = param_beta4_2 * t78;
        let t80 = t65 * t22 / 2.0 + t69 * t12 / 4.0 + 0.125 * t72 * t30 + t79;
        let t84 = 1.0 + t64 / t80 / 2.0;
        let t85 = f64::ln(t84);
        let t87 = 1.0 / param_fz20;
        let t89 = t56 * t57 * t63 * t85 * t87;
        let tzk0 = -2.0 * t45 + 2.0 * t89;
        zk[ip] += tzk0;
        let t94 = t1 * t2 * t3;
        let t96 = 1.0 / t10 / rho[ip];
        let t99 = t94 * t9 * t96 * t44;
        let t101 = t39 * t39;
        let t102 = 1.0 / t101;
        let t103 = t15 * t102;
        let t104 = 1.0 / t22;
        let t106 = t18 * t104 * t3;
        let t107 = t9 * t96;
        let t112 = f64::sqrt(t21);
        let t114 = t29 * t112 * t3;
        let t117 = 1.0 / rho[ip];
        let t121 = -t106 * t107 / 12.0 - t26 * t107 / 12.0 - 0.0625 * t114 * t107 - t38 * t36 * t117 / 3.0;
        let t122 = 1.0 / t43;
        let t123 = t121 * t122;
        let t124 = t103 * t123;
        let t127 = t56 * t57 * t59 * t3;
        let t131 = t127 * t9 * t96 * t85 * t87;
        let t133 = t56 * t63;
        let t134 = t80 * t80;
        let t135 = 1.0 / t134;
        let t137 = t65 * t104 * t3;
        let t143 = t72 * t112 * t3;
        let t149 = -t137 * t107 / 12.0 - t69 * t107 / 12.0 - 0.0625 * t143 * t107 - t79 * t77 * t117 / 3.0;
        let t151 = 1.0 / t84;
        let t152 = t151 * t87;
        let t154 = t133 * t135 * t149 * t152;
        let tvrho0 = -2.0 * t45 + 2.0 * t89 + rho[ip] * (t99 / 6.0 + t124 - t131 / 6.0 - t154);
        vrho[ip] += tvrho0;
        let t161 = rho[ip] * rho[ip];
        let t163 = 1.0 / t10 / t161;
        let t166 = t94 * t9 * t163 * t44;
        let t168 = t4 * t9;
        let t169 = t96 * t102;
        let t171 = t168 * t169 * t123;
        let t173 = t101 * t39;
        let t174 = 1.0 / t173;
        let t175 = t15 * t174;
        let t176 = t121 * t121;
        let t177 = t176 * t122;
        let t178 = t175 * t177;
        let t181 = 1.0 / t22 / t21;
        let t183 = t3 * t3;
        let t184 = t18 * t181 * t183;
        let t185 = t6 * t6;
        let t186 = t185 * t7;
        let t187 = t10 * t10;
        let t190 = t186 / t187 / t161;
        let t193 = t9 * t163;
        let t198 = 1.0/f64::sqrt(t21);
        let t200 = t29 * t198 * t183;
        let t205 = t36 * t36;
        let t206 = 1.0 / t161;
        let t213 = -t184 * t190 / 18.0 + t106 * t193 / 9.0 + t26 * t193 / 9.0 + 0.041666666666666664 * t200 * t190 + 0.08333333333333333 * t114 * t193 + t38 * t205 * t206 / 9.0 + t38 * t36 * t206 / 3.0;
        let t214 = t213 * t122;
        let t215 = t103 * t214;
        let t216 = t101 * t101;
        let t217 = 1.0 / t216;
        let t218 = t15 * t217;
        let t219 = t43 * t43;
        let t220 = 1.0 / t219;
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
        let t240 = 1.0 / t239;
        let t241 = t149 * t149;
        let t244 = t133 * t240 * t241 * t152;
        let t247 = t65 * t181 * t183;
        let t255 = t72 * t198 * t183;
        let t260 = t77 * t77;
        let t267 = -t247 * t190 / 18.0 + t137 * t193 / 9.0 + t69 * t193 / 9.0 + 0.041666666666666664 * t255 * t190 + 0.08333333333333333 * t143 * t193 + t79 * t260 * t206 / 9.0 + t79 * t77 * t206 / 3.0;
        let t270 = t133 * t135 * t267 * t152;
        let t271 = t134 * t134;
        let t272 = 1.0 / t271;
        let t274 = t56 * t63 * t272;
        let t275 = t84 * t84;
        let t276 = 1.0 / t275;
        let t277 = t241 * t276;
        let t278 = t87 * t64;
        let t280 = t274 * t277 * t278;
        let tv2rho20 = t99 / 3.0 + 2.0 * t124 - t131 / 3.0 - 2.0 * t154 + rho[ip] * (-2.0 / 9.0 * t166 - t171 / 6.0 - 2.0 * t178 + t215 + t223 / 2.0 + 2.0 / 9.0 * t228 + t237 / 6.0 + 2.0 * t244 - t270 - t280 / 2.0);
        v2rho2[ip] += tv2rho20;
        let t294 = t96 * t174;
        let t296 = t168 * t294 * t177;
        let t304 = 1.0 / t22 / t183 / t185 / t7 * t187 / 4.0;
        let t305 = t18 * t304;
        let t306 = t161 * t161;
        let t307 = 1.0 / t306;
        let t308 = t5 * t307;
        let t311 = t161 * rho[ip];
        let t314 = t186 / t187 / t311;
        let t318 = 1.0 / t10 / t311;
        let t319 = t9 * t318;
        let t324 = 1.0/pow_3_2(t21);
        let t325 = t29 * t324;
        let t332 = t205 * t36;
        let t333 = 1.0 / t311;
        let t343 = -t305 * t308 / 3.0 + 2.0 / 9.0 * t184 * t314 - 7.0 / 27.0 * t106 * t319 - 7.0 / 27.0 * t26 * t319 + 0.08333333333333333 * t325 * t308 - 0.16666666666666666 * t200 * t314 - 0.19444444444444445 * t114 * t319 - t38 * t332 * t333 / 27.0 - t38 * t205 * t333 / 3.0 - 2.0 / 3.0 * t38 * t36 * t333;
        let t344 = t343 * t122;
        let t345 = t103 * t344;
        let t348 = t220 * t17 * t121;
        let t349 = t218 * t213 * t348;
        let t351 = t176 * t121;
        let t352 = t351 * t122;
        let t353 = t218 * t352;
        let t355 = t123 * t213;
        let t356 = t175 * t355;
        let t359 = 1.0 / t216 / t39;
        let t360 = t15 * t359;
        let t362 = t351 * t220 * t17;
        let t363 = t360 * t362;
        let t366 = 1.0 / t216 / t101;
        let t367 = t15 * t366;
        let t369 = 1.0 / t219 / t43;
        let t371 = t1 * t1;
        let t372 = 1.0 / t371;
        let t373 = t351 * t369 * t372;
        let t374 = t367 * t373;
        let t376 = t65 * t304;
        let t385 = t72 * t324;
        let t392 = t260 * t77;
        let t402 = -t376 * t308 / 3.0 + 2.0 / 9.0 * t247 * t314 - 7.0 / 27.0 * t137 * t319 - 7.0 / 27.0 * t69 * t319 + 0.08333333333333333 * t385 * t308 - 0.16666666666666666 * t255 * t314 - 0.19444444444444445 * t143 * t319 - t79 * t392 * t333 / 27.0 - t79 * t260 * t333 / 3.0 - 2.0 / 3.0 * t79 * t77 * t333;
        let t405 = t133 * t135 * t402 * t152;
        let t408 = t94 * t9 * t318 * t44;
        let t410 = t163 * t102;
        let t412 = t168 * t410 * t123;
        let t415 = t168 * t169 * t214;
        let t417 = t296 / 2.0 + t345 + 3.0 / 2.0 * t349 + 6.0 * t353 - 6.0 * t356 - 3.0 * t363 + t374 / 2.0 - t405 + 14.0 / 27.0 * t408 + t412 / 3.0 - t415 / 4.0;
        let t418 = t241 * t149;
        let t421 = t133 * t272 * t418 * t152;
        let t424 = t56 * t63 * t240;
        let t425 = t87 * t267;
        let t427 = t424 * t234 * t425;
        let t430 = 1.0 / t271 / t80;
        let t432 = t56 * t63 * t430;
        let t435 = t432 * t418 * t276 * t278;
        let t438 = 1.0 / t271 / t134;
        let t440 = t56 * t63 * t438;
        let t442 = 1.0 / t275 / t84;
        let t444 = t57 * t57;
        let t445 = 1.0 / t444;
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
        let t490 = -6.0 * t421 + 6.0 * t427 + 3.0 * t435 - t448 / 2.0 - 14.0 / 27.0 * t453 - t458 / 3.0 + t463 / 4.0 - t467 / 8.0 - 3.0 / 2.0 * t472 - t478 / 2.0 + t488 / 8.0;
        let tv3rho30 = -2.0 / 3.0 * t166 - t171 / 2.0 - 6.0 * t178 + 3.0 * t215 + 3.0 / 2.0 * t223 + 2.0 / 3.0 * t228 + t237 / 2.0 + 6.0 * t244 - 3.0 * t270 - 3.0 / 2.0 * t280 + rho[ip] * (t417 + t490);
        v3rho3[ip] += tv3rho30;
    }
}
