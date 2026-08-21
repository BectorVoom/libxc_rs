//! LDA_K_ZLP kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_zlp_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t9 = rho0 - rho1;
        let t10 = rho0 + rho1;
        let t11 = 1.0 / t10;
        let t12 = t9 * t11;
        let t13 = 1.0 + t12;
        let t14 = t13 <= zeta_threshold;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t17 = t16 * zeta_threshold;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t14, t17, t19 * t13);
        let t22 = 1.0 - t12;
        let t23 = t22 <= zeta_threshold;
        let t24 = pow_1_3(t22);
        let t25 = t24 * t24;
        let t27 = piecewise3(t23, t17, t25 * t22);
        let t29 = t21 / 2.0 + t27 / 2.0;
        let t30 = pow_1_3(t10);
        let t31 = t30 * t30;
        let t32 = t29 * t31;
        let t33 = 1.0 / t30;
        let t35 = 1.0 + 510.2040816326531 * t33;
        let t36 = rmath::ln(t35);
        let t39 = 1.0 - 0.00196 * t30 * t36;
        let t41 = t8 * t32 * t39;
        let tzk0 = 1.0790666666666666 * t41;
        zk[ip] += tzk0;
        let t42 = 1.7984444444444445 * t41;
        let t43 = t31 * t10;
        let t45 = t43 * t2 * t5;
        let t46 = t10 * t10;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t49 = t11 - t48;
        let t52 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t49);
        let t53 = -t49;
        let t56 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t53);
        let t58 = t52 / 2.0 + t56 / 2.0;
        let t59 = t7 * t58;
        let t63 = t7 * t29;
        let t67 = 1.0 / t35;
        let t70 = -0.0006533333333333333 / t31 * t36 + 0.3333333333333333 * t11 * t67;
        let t73 = 1.0790666666666666 * t45 * t63 * t70;
        let tvrho0 = t42 + 1.0790666666666666 * t45 * t59 * t39 + t73;
        vrho[ip * 2] += tvrho0;
        let t74 = -t11 - t48;
        let t77 = piecewise3(t14, 0.0, 5.0 / 3.0 * t19 * t74);
        let t78 = -t74;
        let t81 = piecewise3(t23, 0.0, 5.0 / 3.0 * t25 * t78);
        let t84 = t7 * (t77 / 2.0 + t81 / 2.0);
        let t85 = t84 * t39;
        let tvrho1 = t42 + 1.0790666666666666 * t45 * t85 + t73;
        vrho[ip * 2 + 1] += tvrho1;
        let t88 = t58 * t31;
        let t90 = t8 * t88 * t39;
        let t92 = t29 * t33;
        let t95 = 1.198962962962963 * t8 * t92 * t39;
        let t98 = 3.596888888888889 * t8 * t32 * t70;
        let t99 = 1.0 / t18;
        let t100 = t49 * t49;
        let t103 = t46 * t10;
        let t104 = 1.0 / t103;
        let t105 = t9 * t104;
        let t107 = -2.0 * t47 + 2.0 * t105;
        let t111 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t100 + 5.0 / 3.0 * t19 * t107);
        let t112 = 1.0 / t24;
        let t113 = t53 * t53;
        let t116 = -t107;
        let t120 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t113 + 5.0 / 3.0 * t25 * t116);
        let t122 = t111 / 2.0 + t120 / 2.0;
        let t123 = t7 * t122;
        let t128 = t45 * t59 * t70;
        let t136 = 1.0 / t30 / t46;
        let t137 = t35 * t35;
        let t138 = 1.0 / t137;
        let t141 = 0.00043555555555555557 / t43 * t36 - 0.2222222222222222 * t47 * t67 + 56.68934240362812 * t136 * t138;
        let t144 = 1.0790666666666666 * t45 * t63 * t141;
        let tv2rho20 = 3.596888888888889 * t90 + t95 + t98 + 1.0790666666666666 * t45 * t123 * t39 + 2.1581333333333332 * t128 + t144;
        v2rho2[ip * 3] += tv2rho20;
        let t147 = t31 * t2 * t5;
        let t148 = t147 * t85;
        let t150 = t99 * t74;
        let t153 = t19 * t9;
        let t157 = piecewise3(t14, 0.0, 10.0 / 9.0 * t150 * t49 + 10.0 / 3.0 * t153 * t104);
        let t158 = t112 * t78;
        let t161 = t25 * t9;
        let t165 = piecewise3(t23, 0.0, 10.0 / 9.0 * t158 * t53 - 10.0 / 3.0 * t161 * t104);
        let t168 = t7 * (t157 / 2.0 + t165 / 2.0);
        let t169 = t168 * t39;
        let t172 = t84 * t70;
        let t173 = t45 * t172;
        let tv2rho21 = 1.7984444444444445 * t90 + t95 + t98 + 1.7984444444444445 * t148 + 1.0790666666666666 * t45 * t169 + 1.0790666666666666 * t173 + 1.0790666666666666 * t128 + t144;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t177 = t74 * t74;
        let t181 = 2.0 * t47 + 2.0 * t105;
        let t185 = piecewise3(t14, 0.0, 10.0 / 9.0 * t99 * t177 + 5.0 / 3.0 * t19 * t181);
        let t186 = t78 * t78;
        let t189 = -t181;
        let t193 = piecewise3(t23, 0.0, 10.0 / 9.0 * t112 * t186 + 5.0 / 3.0 * t25 * t189);
        let t196 = t7 * (t185 / 2.0 + t193 / 2.0);
        let t197 = t196 * t39;
        let tv2rho22 = 3.596888888888889 * t148 + t95 + t98 + 1.0790666666666666 * t45 * t197 + 2.1581333333333332 * t173 + t144;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t201 = t122 * t31;
        let t203 = t8 * t201 * t39;
        let t205 = t58 * t33;
        let t207 = t8 * t205 * t39;
        let t210 = t8 * t88 * t70;
        let t213 = 1.0 / t30 / t10;
        let t214 = t29 * t213;
        let t217 = 0.3996543209876543 * t8 * t214 * t39;
        let t220 = 3.596888888888889 * t8 * t92 * t70;
        let t223 = 5.395333333333333 * t8 * t32 * t141;
        let t225 = 1.0 / t18 / t13;
        let t226 = t100 * t49;
        let t229 = t99 * t49;
        let t232 = t46 * t46;
        let t233 = 1.0 / t232;
        let t234 = t9 * t233;
        let t236 = 6.0 * t104 - 6.0 * t234;
        let t240 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t226 + 10.0 / 3.0 * t229 * t107 + 5.0 / 3.0 * t19 * t236);
        let t242 = 1.0 / t24 / t22;
        let t243 = t113 * t53;
        let t246 = t112 * t53;
        let t249 = -t236;
        let t253 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t243 + 10.0 / 3.0 * t246 * t116 + 5.0 / 3.0 * t25 * t249);
        let t255 = t240 / 2.0 + t253 / 2.0;
        let t256 = t7 * t255;
        let t261 = t45 * t123 * t70;
        let t264 = t45 * t59 * t141;
        let t277 = 1.0 / t31 / t103;
        let t279 = 1.0 / t137 / t35;
        let t282 = -0.000725925925925926 / t31 / t46 * t36 + 0.37037037037037035 * t104 * t67 - 170.06802721088437 / t30 / t103 * t138 + 19282.089252934733 * t277 * t279;
        let t285 = 1.0790666666666666 * t45 * t63 * t282;
        let tv3rho30 = 5.395333333333333 * t203 + 3.596888888888889 * t207 + 10.790666666666667 * t210 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t256 * t39 + 3.2372 * t261 + 3.2372 * t264 + t285;
        v3rho3[ip * 4] += tv3rho30;
        let t290 = t33 * t2 * t5;
        let t291 = t290 * t85;
        let t294 = 3.596888888888889 * t147 * t169;
        let t295 = t147 * t172;
        let t297 = t225 * t74;
        let t300 = t99 * t9;
        let t311 = piecewise3(t14, 0.0, -10.0 / 27.0 * t297 * t100 + 40.0 / 9.0 * t300 * t104 * t49 + 10.0 / 9.0 * t150 * t107 + 10.0 / 3.0 * t19 * t104 - 10.0 * t153 * t233);
        let t312 = t242 * t78;
        let t315 = t112 * t9;
        let t326 = piecewise3(t23, 0.0, -10.0 / 27.0 * t312 * t113 - 40.0 / 9.0 * t315 * t104 * t53 + 10.0 / 9.0 * t158 * t116 - 10.0 / 3.0 * t25 * t104 + 10.0 * t161 * t233);
        let t329 = t7 * (t311 / 2.0 + t326 / 2.0);
        let t330 = t329 * t39;
        let t333 = t168 * t70;
        let t335 = 2.1581333333333332 * t45 * t333;
        let t336 = t84 * t141;
        let t337 = t45 * t336;
        let tv3rho31 = 1.7984444444444445 * t203 + 2.397925925925926 * t207 + 7.193777777777778 * t210 - t217 + t220 + t223 + 1.198962962962963 * t291 + t294 + 3.596888888888889 * t295 + 1.0790666666666666 * t45 * t330 + t335 + 1.0790666666666666 * t337 + 1.0790666666666666 * t261 + 2.1581333333333332 * t264 + t285;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let t345 = t147 * t197;
        let t347 = t225 * t177;
        let t352 = t99 * t181;
        let t357 = -2.0 * t104 - 6.0 * t234;
        let t361 = piecewise3(t14, 0.0, -10.0 / 27.0 * t347 * t49 + 40.0 / 9.0 * t150 * t105 + 10.0 / 9.0 * t352 * t49 + 5.0 / 3.0 * t19 * t357);
        let t362 = t242 * t186;
        let t367 = t112 * t189;
        let t370 = -t357;
        let t374 = piecewise3(t23, 0.0, -10.0 / 27.0 * t362 * t53 - 40.0 / 9.0 * t158 * t105 + 10.0 / 9.0 * t367 * t53 + 5.0 / 3.0 * t25 * t370);
        let t377 = t7 * (t361 / 2.0 + t374 / 2.0);
        let t378 = t377 * t39;
        let t381 = t196 * t70;
        let t382 = t45 * t381;
        let tv3rho32 = 2.397925925925926 * t291 + t294 + 7.193777777777778 * t295 + 1.198962962962963 * t207 - t217 + t220 + 3.596888888888889 * t210 + t223 + 1.7984444444444445 * t345 + 1.0790666666666666 * t45 * t378 + 1.0790666666666666 * t382 + t335 + 2.1581333333333332 * t337 + 1.0790666666666666 * t264 + t285;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let t389 = t177 * t74;
        let t395 = -6.0 * t104 - 6.0 * t234;
        let t399 = piecewise3(t14, 0.0, -10.0 / 27.0 * t225 * t389 + 10.0 / 3.0 * t150 * t181 + 5.0 / 3.0 * t19 * t395);
        let t400 = t186 * t78;
        let t405 = -t395;
        let t409 = piecewise3(t23, 0.0, -10.0 / 27.0 * t242 * t400 + 10.0 / 3.0 * t158 * t189 + 5.0 / 3.0 * t25 * t405);
        let t412 = t7 * (t399 / 2.0 + t409 / 2.0);
        let t413 = t412 * t39;
        let tv3rho33 = 3.596888888888889 * t291 + 5.395333333333333 * t345 + 10.790666666666667 * t295 - t217 + t220 + t223 + 1.0790666666666666 * t45 * t413 + 3.2372 * t382 + 3.2372 * t337 + t285;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
