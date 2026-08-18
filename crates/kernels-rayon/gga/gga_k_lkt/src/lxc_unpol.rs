//! GGA_K_LKT lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lkt_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = t24 * t24;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t29 = t25 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = M_CBRT2;
        let t32 = t30 * t31;
        let t34 = 1.0 / t21 / rho[ip];
        let t37 = t29 * t32 * t34 / 12.0;
        let t38 = t37 < 200.0;
        let t39 = piecewise3(t38, t37, 200.0);
        let t40 = param_a * t39;
        let t41 = f64::cosh(t40);
        let t42 = 1.0 / t41;
        let t43 = t27 * t27;
        let t45 = t24 / t43;
        let t46 = t31 * t31;
        let t47 = sigma[ip] * t46;
        let t48 = rho[ip] * rho[ip];
        let t50 = 1.0 / t22 / t48;
        let t54 = t42 + 5.0 / 72.0 * t45 * t47 * t50;
        let t58 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
        let t60 = t20 / t21;
        let t64 = t41 * t41;
        let t65 = 1.0 / t64;
        let t66 = t65 * param_a;
        let t68 = 1.0 / t21 / t48;
        let t72 = piecewise3(t38, -t29 * t32 * t68 / 9.0, 0.0);
        let t73 = f64::sinh(t40);
        let t74 = t72 * t73;
        let t76 = t48 * rho[ip];
        let t78 = 1.0 / t22 / t76;
        let t82 = -t66 * t74 - 5.0 / 27.0 * t45 * t47 * t78;
        let t87 = piecewise3(t2, 0.0, t7 * t60 * t54 / 10.0 + 3.0 / 20.0 * t7 * t23 * t82);
        let tvrho0 = 2.0 * rho[ip] * t87 + 2.0 * t58;
        vrho[ip] += tvrho0;
        let t91 = 1.0 / t30 * t31;
        let t95 = piecewise3(t38, t29 * t91 * t34 / 24.0, 0.0);
        let t96 = t95 * t73;
        let t101 = -t66 * t96 + 5.0 / 72.0 * t45 * t46 * t50;
        let t105 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t101);
        let tvsigma0 = 2.0 * rho[ip] * t105;
        vsigma[ip] += tvsigma0;
        let t108 = t20 * t34;
        let t116 = 1.0 / t64 / t41;
        let t117 = param_a * param_a;
        let t118 = t116 * t117;
        let t119 = t72 * t72;
        let t120 = t73 * t73;
        let t125 = 1.0 / t21 / t76;
        let t129 = piecewise3(t38, 7.0 / 27.0 * t29 * t32 * t125, 0.0);
        let t132 = t42 * t117;
        let t134 = t48 * t48;
        let t136 = 1.0 / t22 / t134;
        let t140 = 2.0 * t118 * t119 * t120 - t66 * t129 * t73 - t132 * t119 + 55.0 / 81.0 * t45 * t47 * t136;
        let t145 = piecewise3(t2, 0.0, -t7 * t108 * t54 / 30.0 + t7 * t60 * t82 / 5.0 + 3.0 / 20.0 * t7 * t23 * t140);
        let tv2rho20 = 2.0 * rho[ip] * t145 + 4.0 * t87;
        v2rho2[ip] += tv2rho20;
        let t151 = t95 * t120;
        let t158 = piecewise3(t38, -t29 * t91 * t68 / 18.0, 0.0);
        let t159 = t158 * t73;
        let t166 = 2.0 * t118 * t151 * t72 - t66 * t159 - t132 * t95 * t72 - 5.0 / 27.0 * t45 * t46 * t78;
        let t171 = piecewise3(t2, 0.0, t7 * t60 * t101 / 10.0 + 3.0 / 20.0 * t7 * t23 * t166);
        let tv2rhosigma0 = 2.0 * rho[ip] * t171 + 2.0 * t105;
        v2rhosigma[ip] += tv2rhosigma0;
        let t174 = t95 * t95;
        let t175 = t174 * t120;
        let t180 = 1.0 / t30 / sigma[ip] * t31;
        let t184 = piecewise3(t38, -t29 * t180 * t34 / 48.0, 0.0);
        let t185 = t184 * t73;
        let t188 = 2.0 * t118 * t175 - t132 * t174 - t66 * t185;
        let t192 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t188);
        let tv2sigma20 = 2.0 * rho[ip] * t192;
        v2sigma2[ip] += tv2sigma20;
        let t195 = t20 * t68;
        let t205 = t64 * t64;
        let t207 = t117 * param_a;
        let t208 = 1.0 / t205 * t207;
        let t209 = t119 * t72;
        let t210 = t120 * t73;
        let t214 = t72 * t120;
        let t218 = t65 * t207;
        let t223 = 1.0 / t21 / t134;
        let t227 = piecewise3(t38, -70.0 / 81.0 * t29 * t32 * t223, 0.0);
        let t233 = t134 * rho[ip];
        let t235 = 1.0 / t22 / t233;
        let t239 = -6.0 * t208 * t209 * t210 + 6.0 * t118 * t214 * t129 + 5.0 * t218 * t209 * t73 - t66 * t227 * t73 - 3.0 * t132 * t129 * t72 - 770.0 / 243.0 * t45 * t47 * t235;
        let t244 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t195 * t54 - t7 * t108 * t82 / 10.0 + 3.0 / 10.0 * t7 * t60 * t140 + 3.0 / 20.0 * t7 * t23 * t239);
        let tv3rho30 = 2.0 * rho[ip] * t244 + 6.0 * t145;
        v3rho3[ip] += tv3rho30;
        let t258 = t158 * t120;
        let t271 = piecewise3(t38, 7.0 / 54.0 * t29 * t91 * t125, 0.0);
        let t282 = -6.0 * t208 * t95 * t210 * t119 + 4.0 * t118 * t258 * t72 + 5.0 * t218 * t96 * t119 + 2.0 * t118 * t151 * t129 - t66 * t271 * t73 - 2.0 * t132 * t158 * t72 - t132 * t95 * t129 + 55.0 / 81.0 * t45 * t46 * t136;
        let t287 = piecewise3(t2, 0.0, -t7 * t108 * t101 / 30.0 + t7 * t60 * t166 / 5.0 + 3.0 / 20.0 * t7 * t23 * t282);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t287 + 4.0 * t171;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t293 = t174 * t210;
        let t300 = t174 * t73;
        let t304 = t184 * t120;
        let t311 = piecewise3(t38, t29 * t180 * t68 / 36.0, 0.0);
        let t319 = 4.0 * t118 * t151 * t158 + 2.0 * t118 * t304 * t72 - 2.0 * t132 * t95 * t158 - t132 * t184 * t72 - 6.0 * t208 * t293 * t72 + 5.0 * t218 * t300 * t72 - t66 * t311 * t73;
        let t324 = piecewise3(t2, 0.0, t7 * t60 * t188 / 10.0 + 3.0 / 20.0 * t7 * t23 * t319);
        let tv3rhosigma20 = 2.0 * rho[ip] * t324 + 2.0 * t192;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t327 = t174 * t95;
        let t337 = sigma[ip] * sigma[ip];
        let t340 = 1.0 / t30 / t337 * t31;
        let t344 = piecewise3(t38, t29 * t340 * t34 / 32.0, 0.0);
        let t350 = 6.0 * t118 * t151 * t184 - 3.0 * t132 * t184 * t95 - 6.0 * t208 * t327 * t210 + 5.0 * t218 * t327 * t73 - t66 * t344 * t73;
        let t354 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t350);
        let tv3sigma30 = 2.0 * rho[ip] * t354;
        v3sigma3[ip] += tv3sigma30;
        let t372 = t117 * t117;
        let t373 = 1.0 / t205 / t41 * t372;
        let t374 = t119 * t119;
        let t375 = t120 * t120;
        let t383 = t116 * t372;
        let t387 = t129 * t129;
        let t398 = t42 * t372;
        let t406 = piecewise3(t38, 910.0 / 243.0 * t29 * t32 / t21 / t233, 0.0);
        let t420 = 24.0 * t373 * t374 * t375 - 36.0 * t208 * t119 * t210 * t129 - 28.0 * t383 * t374 * t120 + 6.0 * t118 * t387 * t120 + 30.0 * t218 * t119 * t73 * t129 + 8.0 * t118 * t214 * t227 + 5.0 * t398 * t374 - t66 * t406 * t73 - 4.0 * t132 * t227 * t72 - 3.0 * t132 * t387 + 13090.0 / 729.0 * t45 * t47 / t22 / t134 / t48;
        let t425 = piecewise3(t2, 0.0, -14.0 / 135.0 * t7 * t20 * t125 * t54 + 8.0 / 45.0 * t7 * t195 * t82 - t7 * t108 * t140 / 5.0 + 2.0 / 5.0 * t7 * t60 * t239 + 3.0 / 20.0 * t7 * t23 * t420);
        let tv4rho40 = 2.0 * rho[ip] * t425 + 8.0 * t244;
        v4rho4[ip] += tv4rho40;
        let t448 = t208 * t95;
        let t449 = t210 * t72;
        let t453 = t218 * t95;
        let t480 = piecewise3(t38, -35.0 / 81.0 * t29 * t91 * t223, 0.0);
        let t491 = 24.0 * t373 * t95 * t375 * t209 - 28.0 * t383 * t151 * t209 + 15.0 * t218 * t159 * t119 - 18.0 * t448 * t449 * t129 + 15.0 * t453 * t74 * t129 - 3.0 * t132 * t271 * t72 + 6.0 * t118 * t271 * t120 * t72 + 6.0 * t118 * t258 * t129 + 2.0 * t118 * t151 * t227 - 18.0 * t208 * t158 * t210 * t119 + 5.0 * t398 * t95 * t209 - t66 * t480 * t73 - 3.0 * t132 * t158 * t129 - t132 * t95 * t227 - 770.0 / 243.0 * t45 * t46 * t235;
        let t496 = piecewise3(t2, 0.0, 2.0 / 45.0 * t7 * t195 * t101 - t7 * t108 * t166 / 10.0 + 3.0 / 10.0 * t7 * t60 * t282 + 3.0 / 20.0 * t7 * t23 * t491);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t496 + 6.0 * t287;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t526 = t184 * t210;
        let t542 = t158 * t158;
        let t555 = piecewise3(t38, -7.0 / 108.0 * t29 * t180 * t125, 0.0);
        let t565 = 5.0 * t218 * t300 * t129 + 4.0 * t118 * t311 * t120 * t72 + 2.0 * t118 * t304 * t129 + 24.0 * t373 * t174 * t375 * t119 - 28.0 * t383 * t175 * t119 + 20.0 * t453 * t159 * t72 - 6.0 * t208 * t526 * t119 + 5.0 * t218 * t185 * t119 - 24.0 * t448 * t449 * t158 - 2.0 * t132 * t311 * t72 - 6.0 * t208 * t293 * t129 + 4.0 * t118 * t542 * t120 + 4.0 * t118 * t151 * t271 + 5.0 * t398 * t174 * t119 - t66 * t555 * t73 - t132 * t184 * t129 - 2.0 * t132 * t542 - 2.0 * t132 * t95 * t271;
        let t570 = piecewise3(t2, 0.0, -t7 * t108 * t188 / 30.0 + t7 * t60 * t319 / 5.0 + 3.0 / 20.0 * t7 * t23 * t565);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t570 + 4.0 * t324;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t612 = piecewise3(t38, -t29 * t340 * t68 / 24.0, 0.0);
        let t623 = 2.0 * t118 * t344 * t120 * t72 - 28.0 * t383 * t327 * t120 * t72 + 24.0 * t373 * t327 * t375 * t72 + 6.0 * t118 * t151 * t311 + 6.0 * t118 * t258 * t184 - 3.0 * t132 * t184 * t158 - 3.0 * t132 * t311 * t95 - t132 * t344 * t72 - 18.0 * t208 * t293 * t158 + 15.0 * t218 * t300 * t158 + 15.0 * t453 * t185 * t72 + 5.0 * t398 * t327 * t72 - 18.0 * t448 * t526 * t72 - t66 * t612 * t73;
        let t628 = piecewise3(t2, 0.0, t7 * t60 * t350 / 10.0 + 3.0 / 20.0 * t7 * t23 * t623);
        let tv4rhosigma30 = 2.0 * rho[ip] * t628 + 2.0 * t354;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t631 = t174 * t174;
        let t641 = t184 * t184;
        let t660 = piecewise3(t38, -5.0 / 64.0 * t29 / t30 / t337 / sigma[ip] * t31 * t34, 0.0);
        let t672 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * (6.0 * t118 * t641 * t120 + 8.0 * t118 * t151 * t344 - 28.0 * t383 * t631 * t120 - 4.0 * t132 * t344 * t95 - 36.0 * t208 * t293 * t184 + 30.0 * t218 * t300 * t184 + 24.0 * t373 * t631 * t375 - t66 * t660 * t73 - 3.0 * t132 * t641 + 5.0 * t398 * t631));
        let tv4sigma40 = 2.0 * rho[ip] * t672;
        v4sigma4[ip] += tv4sigma40;
    }
}
