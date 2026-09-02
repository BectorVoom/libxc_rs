//! GGA_X_SOGGA11 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sogga11.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sogga11_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_a_1: f64,
    param_mu: f64,
    param_kappa: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_a_0: f64,
    param_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t21 = param_a_1;
        let t22 = M_CBRT6;
        let t23 = param_mu * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = t25 * t25;
        let t27 = 1.0 / t26;
        let t28 = t23 * t27;
        let t29 = 1.0 / param_kappa;
        let t30 = t29 * sigma[ip];
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = rho[ip] * rho[ip];
        let t34 = t18 * t18;
        let t36 = 1.0 / t34 / t33;
        let t37 = t32 * t36;
        let t40 = t28 * t30 * t37 / 24.0;
        let t41 = 1.0 + t40;
        let t43 = 1.0 - 1.0 / t41;
        let t45 = param_a_2;
        let t46 = t43 * t43;
        let t48 = param_a_3;
        let t49 = t46 * t43;
        let t51 = param_a_4;
        let t52 = t46 * t46;
        let t54 = param_a_5;
        let t58 = param_b_1;
        let t59 = rmath::exp(-t40);
        let t60 = 1.0 - t59;
        let t62 = param_b_2;
        let t63 = t60 * t60;
        let t65 = param_b_3;
        let t66 = t63 * t60;
        let t68 = param_b_4;
        let t69 = t63 * t63;
        let t71 = param_b_5;
        let t74 = t54 * t52 * t43 + t71 * t69 * t60 + t21 * t43 + t45 * t46 + t48 * t49 + t51 * t52 + t58 * t60 + t62 * t63 + t65 * t66 + t68 * t69 + param_a_0 + param_b_0;
        let t78 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t17 / t34;
        let t84 = t41 * t41;
        let t85 = 1.0 / t84;
        let t87 = t21 * t85 * t23;
        let t88 = t27 * t29;
        let t89 = sigma[ip] * t32;
        let t90 = t33 * rho[ip];
        let t92 = 1.0 / t34 / t90;
        let t94 = t88 * t89 * t92;
        let t97 = t45 * t43;
        let t98 = t85 * param_mu;
        let t99 = t98 * t22;
        let t100 = t97 * t99;
        let t103 = t48 * t46;
        let t104 = t103 * t99;
        let t107 = t51 * t49;
        let t108 = t107 * t99;
        let t111 = t54 * t52;
        let t112 = t111 * t99;
        let t116 = t22 * t27;
        let t117 = t58 * param_mu * t116;
        let t118 = t32 * t92;
        let t119 = t118 * t59;
        let t120 = t30 * t119;
        let t123 = t62 * t60;
        let t124 = t123 * t28;
        let t127 = t65 * t63;
        let t128 = t127 * t28;
        let t131 = t68 * t66;
        let t132 = t131 * t28;
        let t135 = t71 * t69;
        let t136 = t135 * t28;
        let t139 = -t87 * t94 / 9.0 - 2.0 / 9.0 * t100 * t94 - t104 * t94 / 3.0 - 4.0 / 9.0 * t108 * t94 - 5.0 / 9.0 * t112 * t94 - t117 * t120 / 9.0 - 2.0 / 9.0 * t124 * t120 - t128 * t120 / 3.0 - 4.0 / 9.0 * t132 * t120 - 5.0 / 9.0 * t136 * t120;
        let t144 = piecewise3(t2, 0.0, -t6 * t80 * t74 / 8.0 - 3.0 / 8.0 * t6 * t19 * t139);
        let tvrho0 = 2.0 * rho[ip] * t144 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t150 = t97 * t98;
        let t151 = t29 * t32;
        let t153 = t116 * t151 * t36;
        let t156 = t103 * t98;
        let t159 = t107 * t98;
        let t162 = t111 * t98;
        let t169 = t123 * t23;
        let t171 = t88 * t37 * t59;
        let t174 = t127 * t23;
        let t177 = t131 * t23;
        let t180 = t135 * t23;
        let t183 = t87 * t88 * t37 / 24.0 + t150 * t153 / 12.0 + t156 * t153 / 8.0 + t159 * t153 / 6.0 + 5.0 / 24.0 * t162 * t153 + t117 * t151 * t36 * t59 / 24.0 + t169 * t171 / 12.0 + t174 * t171 / 8.0 + t177 * t171 / 6.0 + 5.0 / 24.0 * t180 * t171;
        let t187 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t183);
        let tvsigma0 = 2.0 * rho[ip] * t187;
        vsigma[ip] += tvsigma0;
        let t192 = t17 / t34 / rho[ip];
        let t199 = t84 * t41;
        let t200 = 1.0 / t199;
        let t201 = param_mu * param_mu;
        let t202 = t200 * t201;
        let t203 = t22 * t22;
        let t204 = t202 * t203;
        let t205 = t97 * t204;
        let t207 = 1.0 / t25 / t24;
        let t208 = param_kappa * param_kappa;
        let t209 = 1.0 / t208;
        let t210 = t207 * t209;
        let t211 = sigma[ip] * sigma[ip];
        let t212 = t211 * t31;
        let t213 = t33 * t33;
        let t216 = 1.0 / t18 / t213 / t90;
        let t218 = t210 * t212 * t216;
        let t221 = t48 * t43;
        let t222 = t84 * t84;
        let t223 = 1.0 / t222;
        let t224 = t223 * t201;
        let t225 = t224 * t203;
        let t226 = t221 * t225;
        let t229 = t103 * t204;
        let t232 = t51 * t46;
        let t233 = t232 * t225;
        let t237 = 1.0 / t34 / t213;
        let t238 = t32 * t237;
        let t239 = t238 * t59;
        let t240 = t30 * t239;
        let t248 = t88 * t89 * t237;
        let t259 = t65 * t60;
        let t260 = t201 * t203;
        let t261 = t260 * t207;
        let t262 = t259 * t261;
        let t263 = t209 * t211;
        let t264 = t31 * t216;
        let t265 = t59 * t59;
        let t266 = t264 * t265;
        let t267 = t263 * t266;
        let t270 = t127 * t261;
        let t271 = t264 * t59;
        let t272 = t263 * t271;
        let t275 = -8.0 / 81.0 * t205 * t218 + 4.0 / 27.0 * t226 * t218 - 4.0 / 27.0 * t229 * t218 + 8.0 / 27.0 * t233 * t218 + 11.0 / 9.0 * t128 * t240 + 44.0 / 27.0 * t132 * t240 + 55.0 / 27.0 * t136 * t240 + 55.0 / 27.0 * t112 * t248 + 22.0 / 27.0 * t124 * t240 + 22.0 / 27.0 * t100 * t248 + 11.0 / 9.0 * t104 * t248 + 44.0 / 27.0 * t108 * t248 + 4.0 / 27.0 * t262 * t267 - 2.0 / 27.0 * t270 * t272;
        let t276 = t68 * t63;
        let t277 = t276 * t261;
        let t280 = t131 * t261;
        let t283 = t71 * t66;
        let t284 = t283 * t261;
        let t287 = t135 * t261;
        let t290 = t107 * t204;
        let t293 = t54 * t49;
        let t294 = t293 * t225;
        let t297 = t111 * t204;
        let t300 = t123 * t261;
        let t308 = t21 * t200 * t260;
        let t312 = t45 * t223 * t260;
        let t316 = t203 * t207;
        let t317 = t58 * t201 * t316;
        let t321 = t62 * t201 * t316;
        let t324 = 8.0 / 27.0 * t277 * t267 - 8.0 / 81.0 * t280 * t272 + 40.0 / 81.0 * t284 * t267 - 10.0 / 81.0 * t287 * t272 - 16.0 / 81.0 * t290 * t218 + 40.0 / 81.0 * t294 * t218 - 20.0 / 81.0 * t297 * t218 - 4.0 / 81.0 * t300 * t272 + 11.0 / 27.0 * t117 * t240 + 11.0 / 27.0 * t87 * t248 - 4.0 / 81.0 * t308 * t218 + 4.0 / 81.0 * t312 * t218 - 2.0 / 81.0 * t317 * t272 + 4.0 / 81.0 * t321 * t267;
        let t325 = t275 + t324;
        let t330 = piecewise3(t2, 0.0, t6 * t192 * t74 / 12.0 - t6 * t80 * t139 / 4.0 - 3.0 / 8.0 * t6 * t19 * t325);
        let tv2rho20 = 2.0 * rho[ip] * t330 + 4.0 * t144;
        v2rho2[ip] += tv2rho20;
        let t343 = t209 * t31;
        let t344 = t213 * t33;
        let t346 = 1.0 / t18 / t344;
        let t347 = t346 * t265;
        let t349 = t343 * t347 * sigma[ip];
        let t354 = t343 * t346 * sigma[ip] * t59;
        let t365 = t31 * t346;
        let t367 = t210 * t365 * sigma[ip];
        let t380 = -t87 * t88 * t118 / 9.0 - t117 * t151 * t92 * t59 / 9.0 - 5.0 / 27.0 * t284 * t349 + 5.0 / 108.0 * t287 * t354 - t262 * t349 / 18.0 + t270 * t354 / 36.0 - t277 * t349 / 9.0 + t280 * t354 / 27.0 - t233 * t367 / 9.0 + 2.0 / 27.0 * t290 * t367 - 5.0 / 27.0 * t294 * t367 + 5.0 / 54.0 * t297 * t367 + t300 * t354 / 54.0 + t205 * t367 / 27.0;
        let t385 = t88 * t119;
        let t391 = t116 * t151 * t92;
        let t412 = -t226 * t367 / 18.0 + t229 * t367 / 18.0 - 4.0 / 9.0 * t177 * t385 - 5.0 / 9.0 * t180 * t385 - 4.0 / 9.0 * t159 * t391 - 5.0 / 9.0 * t162 * t391 - 2.0 / 9.0 * t169 * t385 - 2.0 / 9.0 * t150 * t391 - t156 * t391 / 3.0 + t308 * t367 / 54.0 - t312 * t367 / 54.0 + t317 * t354 / 108.0 - t321 * t349 / 54.0 - t174 * t385 / 3.0;
        let t413 = t380 + t412;
        let t418 = piecewise3(t2, 0.0, -t6 * t80 * t183 / 8.0 - 3.0 / 8.0 * t6 * t19 * t413);
        let tv2rhosigma0 = 2.0 * rho[ip] * t418 + 2.0 * t187;
        v2rhosigma[ip] += tv2rhosigma0;
        let t421 = t213 * rho[ip];
        let t423 = 1.0 / t18 / t421;
        let t424 = t31 * t423;
        let t425 = t210 * t424;
        let t430 = t97 * t202;
        let t432 = t316 * t343 * t423;
        let t435 = t221 * t224;
        let t438 = t103 * t202;
        let t441 = t232 * t224;
        let t444 = t107 * t202;
        let t447 = t293 * t224;
        let t450 = t111 * t202;
        let t461 = t123 * t260;
        let t463 = t210 * t424 * t59;
        let t466 = t259 * t260;
        let t468 = t210 * t424 * t265;
        let t471 = t127 * t260;
        let t474 = t276 * t260;
        let t477 = t131 * t260;
        let t480 = t283 * t260;
        let t483 = t135 * t260;
        let t486 = -t308 * t425 / 144.0 + t312 * t425 / 144.0 - t430 * t432 / 72.0 + t435 * t432 / 48.0 - t438 * t432 / 48.0 + t441 * t432 / 24.0 - t444 * t432 / 36.0 + 5.0 / 72.0 * t447 * t432 - 5.0 / 144.0 * t450 * t432 - t317 * t343 * t423 * t59 / 288.0 + t321 * t343 * t423 * t265 / 144.0 - t461 * t463 / 144.0 + t466 * t468 / 48.0 - t471 * t463 / 96.0 + t474 * t468 / 24.0 - t477 * t463 / 72.0 + 5.0 / 72.0 * t480 * t468 - 5.0 / 288.0 * t483 * t463;
        let t490 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t486);
        let tv2sigma20 = 2.0 * rho[ip] * t490;
        v2sigma2[ip] += tv2sigma20;
    }
}
