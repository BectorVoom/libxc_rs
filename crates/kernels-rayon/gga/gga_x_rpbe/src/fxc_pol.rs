//! GGA_X_RPBE fxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rpbe.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_rpbe_fxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_rpbe_kappa: f64,
    param_rpbe_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = param_rpbe_mu * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t41 = 1.0 / param_rpbe_kappa;
        let t45 = f64::exp(-t34 * sigma0 * t39 * t41 / 24.0);
        let t48 = 1.0 + param_rpbe_kappa * (1.0 - t45);
        let t52 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(t57);
        let t61 = piecewise3(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = rho1 * rho1;
        let t64 = pow_1_3(rho1);
        let t65 = t64 * t64;
        let t67 = 1.0 / t65 / t63;
        let t72 = f64::exp(-t34 * sigma2 * t67 * t41 / 24.0);
        let t75 = 1.0 + param_rpbe_kappa * (1.0 - t72);
        let t79 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t75);
        let tzk0 = t52 + t79;
        zk[ip] += tzk0;
        let t80 = t6 * t6;
        let t81 = 1.0 / t80;
        let t82 = t16 * t81;
        let t84 = piecewise5(t10, 0.0, t14, 0.0, t7 - t82);
        let t87 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t84);
        let t88 = t87 * t26;
        let t92 = t26 * t26;
        let t93 = 1.0 / t92;
        let t94 = t25 * t93;
        let t97 = t5 * t94 * t48 / 8.0;
        let t99 = t5 * t27 * param_rpbe_mu;
        let t100 = t28 * t33;
        let t101 = t35 * rho0;
        let t103 = 1.0 / t37 / t101;
        let t106 = t100 * sigma0 * t103 * t45;
        let t110 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t88 * t48 - t97 + t99 * t106 / 24.0);
        let t111 = t54 * t81;
        let t113 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t111);
        let t116 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t113);
        let t117 = t116 * t26;
        let t121 = t61 * t93;
        let t124 = t5 * t121 * t75 / 8.0;
        let t126 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t117 * t75 - t124);
        let tvrho0 = t52 + t79 + t6 * (t110 + t126);
        vrho[ip * 2] += tvrho0;
        let t130 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t82);
        let t133 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t130);
        let t134 = t133 * t26;
        let t139 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t134 * t48 - t97);
        let t141 = piecewise5(t14, 0.0, t10, 0.0, t7 - t111);
        let t144 = piecewise3(t58, 0.0, 4.0 / 3.0 * t59 * t141);
        let t145 = t144 * t26;
        let t150 = t5 * t62 * param_rpbe_mu;
        let t151 = t63 * rho1;
        let t153 = 1.0 / t65 / t151;
        let t156 = t100 * sigma2 * t153 * t72;
        let t160 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t145 * t75 - t124 + t150 * t156 / 24.0);
        let tvrho1 = t52 + t79 + t6 * (t139 + t160);
        vrho[ip * 2 + 1] += tvrho1;
        let t163 = t5 * t27;
        let t166 = t29 * t33 * t39 * t45;
        let t169 = piecewise3(t1, 0.0, -t163 * t166 / 64.0);
        let tvsigma0 = t6 * t169;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t170 = t5 * t62;
        let t173 = t29 * t33 * t67 * t72;
        let t176 = piecewise3(t53, 0.0, -t170 * t173 / 64.0);
        let tvsigma2 = t6 * t176;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t179 = t23 * t23;
        let t180 = 1.0 / t179;
        let t181 = t84 * t84;
        let t184 = t80 * t6;
        let t185 = 1.0 / t184;
        let t186 = t16 * t185;
        let t189 = piecewise5(t10, 0.0, t14, 0.0, -2.0 * t81 + 2.0 * t186);
        let t193 = piecewise3(t20, 0.0, 4.0 / 9.0 * t180 * t181 + 4.0 / 3.0 * t23 * t189);
        let t194 = t193 * t26;
        let t198 = t87 * t93;
        let t200 = t5 * t198 * t48;
        let t203 = t5 * t88 * param_rpbe_mu;
        let t207 = 1.0 / t92 / t6;
        let t208 = t25 * t207;
        let t211 = t5 * t208 * t48 / 12.0;
        let t213 = t5 * t94 * param_rpbe_mu;
        let t214 = t213 * t106;
        let t216 = t35 * t35;
        let t218 = 1.0 / t37 / t216;
        let t221 = t100 * sigma0 * t218 * t45;
        let t224 = param_rpbe_mu * param_rpbe_mu;
        let t226 = t5 * t27 * t224;
        let t227 = t28 * t28;
        let t230 = t227 / t31 / t30;
        let t231 = sigma0 * sigma0;
        let t232 = t230 * t231;
        let t235 = 1.0 / t36 / t216 / t101;
        let t237 = t235 * t41 * t45;
        let t238 = t232 * t237;
        let t242 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t194 * t48 - t200 / 4.0 + t203 * t106 / 12.0 + t211 + t214 / 36.0 - 11.0 / 72.0 * t99 * t221 + t226 * t238 / 216.0);
        let t243 = t59 * t59;
        let t244 = 1.0 / t243;
        let t245 = t113 * t113;
        let t248 = t54 * t185;
        let t251 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t81 + 2.0 * t248);
        let t255 = piecewise3(t58, 0.0, 4.0 / 9.0 * t244 * t245 + 4.0 / 3.0 * t59 * t251);
        let t256 = t255 * t26;
        let t260 = t116 * t93;
        let t262 = t5 * t260 * t75;
        let t264 = t61 * t207;
        let t267 = t5 * t264 * t75 / 12.0;
        let t269 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t256 * t75 - t262 / 4.0 + t267);
        let tv2rho20 = 2.0 * t110 + 2.0 * t126 + t6 * (t242 + t269);
        v2rho2[ip * 3] += tv2rho20;
        let t272 = t180 * t130;
        let t276 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t186);
        let t280 = piecewise3(t20, 0.0, 4.0 / 9.0 * t272 * t84 + 4.0 / 3.0 * t23 * t276);
        let t281 = t280 * t26;
        let t285 = t133 * t93;
        let t287 = t5 * t285 * t48;
        let t290 = t5 * t134 * param_rpbe_mu;
        let t296 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t281 * t48 - t287 / 8.0 + t290 * t106 / 24.0 - t200 / 8.0 + t211 + t214 / 72.0);
        let t297 = t244 * t141;
        let t301 = piecewise5(t14, 0.0, t10, 0.0, 2.0 * t248);
        let t305 = piecewise3(t58, 0.0, 4.0 / 9.0 * t297 * t113 + 4.0 / 3.0 * t59 * t301);
        let t306 = t305 * t26;
        let t310 = t144 * t93;
        let t312 = t5 * t310 * t75;
        let t316 = t5 * t117 * param_rpbe_mu;
        let t320 = t5 * t121 * param_rpbe_mu;
        let t321 = t320 * t156;
        let t324 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t306 * t75 - t312 / 8.0 - t262 / 8.0 + t267 + t316 * t156 / 24.0 + t321 / 72.0);
        let tv2rho21 = t110 + t126 + t139 + t160 + t6 * (t296 + t324);
        v2rho2[ip * 3 + 1] += tv2rho21;
        let t329 = t130 * t130;
        let t334 = piecewise5(t10, 0.0, t14, 0.0, 2.0 * t81 + 2.0 * t186);
        let t338 = piecewise3(t20, 0.0, 4.0 / 9.0 * t180 * t329 + 4.0 / 3.0 * t23 * t334);
        let t339 = t338 * t26;
        let t345 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t339 * t48 - t287 / 4.0 + t211);
        let t346 = t141 * t141;
        let t351 = piecewise5(t14, 0.0, t10, 0.0, -2.0 * t81 + 2.0 * t248);
        let t355 = piecewise3(t58, 0.0, 4.0 / 9.0 * t244 * t346 + 4.0 / 3.0 * t59 * t351);
        let t356 = t355 * t26;
        let t362 = t5 * t145 * param_rpbe_mu;
        let t366 = t63 * t63;
        let t368 = 1.0 / t65 / t366;
        let t371 = t100 * sigma2 * t368 * t72;
        let t375 = t5 * t62 * t224;
        let t376 = sigma2 * sigma2;
        let t377 = t230 * t376;
        let t380 = 1.0 / t64 / t366 / t151;
        let t382 = t380 * t41 * t72;
        let t383 = t377 * t382;
        let t387 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t356 * t75 - t312 / 4.0 + t362 * t156 / 12.0 + t267 + t321 / 36.0 - 11.0 / 72.0 * t150 * t371 + t375 * t383 / 216.0);
        let tv2rho22 = 2.0 * t139 + 2.0 * t160 + t6 * (t345 + t387);
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t390 = t5 * t88;
        let t393 = t5 * t94;
        let t395 = t393 * t166 / 192.0;
        let t398 = t29 * t33 * t103 * t45;
        let t401 = t216 * t35;
        let t403 = 1.0 / t36 / t401;
        let t406 = sigma0 * t41 * t45;
        let t407 = t230 * t403 * t406;
        let t411 = piecewise3(t1, 0.0, -t390 * t166 / 64.0 - t395 + t163 * t398 / 24.0 - t226 * t407 / 576.0);
        let tv2rhosigma0 = t6 * t411 + t169;
        v2rhosigma[ip * 6] += tv2rhosigma0;
        let tv2rhosigma1 = 0.0;
        v2rhosigma[ip * 6 + 1] += tv2rhosigma1;
        let t413 = t5 * t117;
        let t416 = t5 * t121;
        let t418 = t416 * t173 / 192.0;
        let t420 = piecewise3(t53, 0.0, -t413 * t173 / 64.0 - t418);
        let tv2rhosigma2 = t6 * t420 + t176;
        v2rhosigma[ip * 6 + 2] += tv2rhosigma2;
        let t422 = t5 * t134;
        let t426 = piecewise3(t1, 0.0, -t422 * t166 / 64.0 - t395);
        let tv2rhosigma3 = t6 * t426 + t169;
        v2rhosigma[ip * 6 + 3] += tv2rhosigma3;
        let tv2rhosigma4 = 0.0;
        v2rhosigma[ip * 6 + 4] += tv2rhosigma4;
        let t428 = t5 * t145;
        let t433 = t29 * t33 * t153 * t72;
        let t436 = t366 * t63;
        let t438 = 1.0 / t64 / t436;
        let t441 = sigma2 * t41 * t72;
        let t442 = t230 * t438 * t441;
        let t446 = piecewise3(t53, 0.0, -t428 * t173 / 64.0 - t418 + t170 * t433 / 24.0 - t375 * t442 / 576.0);
        let tv2rhosigma5 = t6 * t446 + t176;
        v2rhosigma[ip * 6 + 5] += tv2rhosigma5;
        let t448 = t216 * rho0;
        let t453 = t230 / t36 / t448 * t41 * t45;
        let t456 = piecewise3(t1, 0.0, t226 * t453 / 1536.0);
        let tv2sigma20 = t6 * t456;
        v2sigma2[ip * 6] += tv2sigma20;
        let tv2sigma21 = 0.0;
        v2sigma2[ip * 6 + 1] += tv2sigma21;
        let tv2sigma22 = 0.0;
        v2sigma2[ip * 6 + 2] += tv2sigma22;
        let tv2sigma23 = 0.0;
        v2sigma2[ip * 6 + 3] += tv2sigma23;
        let tv2sigma24 = 0.0;
        v2sigma2[ip * 6 + 4] += tv2sigma24;
        let t457 = t366 * rho1;
        let t462 = t230 / t64 / t457 * t41 * t72;
        let t465 = piecewise3(t53, 0.0, t375 * t462 / 1536.0);
        let tv2sigma25 = t6 * t465;
        v2sigma2[ip * 6 + 5] += tv2sigma25;
    }
}
