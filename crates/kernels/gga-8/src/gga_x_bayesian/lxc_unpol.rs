//! GGA_X_BAYESIAN lxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 39 shared lines across all orders.
//! Delta: 54 lines unique to lxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_x_bayesian_lxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v4rho4: &mut Array<f64>,
    v4rho3sigma: &mut Array<f64>,
    v4rho2sigma2: &mut Array<f64>,
    v4rhosigma3: &mut Array<f64>,
    v4sigma4: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (39 lines) ---
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t20 * t20;
        let t35 = 1.0 / t22;
        let t36 = t34 * t35;
        let t37 = f64::sqrt(sigma[ip]);
        let t44 = 1.0 + t36 * t37 * t27 / t18 / rho[ip] / 12.0;
        let t45 = t44 * t44;
        let t46 = 1.0 / t45;
        let t47 = t33 * t46;
        let t50 = 0.1926e0 + 0.79008333333333333333e-1 * t26 * t47;
        let t51 = t46 * t50;
        let t55 = 0.10008e1 + t26 * t33 * t51 / 24.0;
        let t59 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t55);
        let tzk0 = 2.0 * t59;
        zk[ip] += tzk0;
        // --- vxc delta (28 lines) ---
        let t61 = t17 / t30;
        let t65 = t29 * rho[ip];
        let t67 = 1.0 / t30 / t65;
        let t68 = t28 * t67;
        let t72 = 1.0 / t21;
        let t73 = t37 * sigma[ip];
        let t74 = t72 * t73;
        let t75 = t29 * t29;
        let t76 = t75 * rho[ip];
        let t77 = 1.0 / t76;
        let t79 = 1.0 / t45 / t44;
        let t80 = t77 * t79;
        let t84 = t68 * t46;
        let t89 = -0.21068888888888888889e0 * t26 * t84 + 0.21068888888888888889e0 * t74 * t80;
        let t90 = t46 * t89;
        let t94 = -t26 * t68 * t51 / 9.0 + t74 * t80 * t50 / 9.0 + t26 * t33 * t90 / 24.0;
        let t99 = piecewise3(t2, 0.0, -t6 * t61 * t55 / 8.0 - 3.0 / 8.0 * t6 * t19 * t94);
        let tvrho0 = 2.0 * rho[ip] * t99 + 2.0 * t59;
        vrho[ip] += tvrho0;
        let t102 = t25 * t28;
        let t103 = t32 * t46;
        let t106 = t72 * t37;
        let t107 = 1.0 / t75;
        let t108 = t107 * t79;
        let t115 = 0.79008333333333333333e-1 * t25 * t47 - 0.79008333333333333333e-1 * t106 * t108;
        let t116 = t46 * t115;
        let t120 = t102 * t103 * t50 / 24.0 - t106 * t108 * t50 / 24.0 + t26 * t33 * t116 / 24.0;
        let t124 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t120);
        let tvsigma0 = 2.0 * rho[ip] * t124;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (47 lines) ---
        let t129 = t17 / t30 / rho[ip];
        let t137 = 1.0 / t30 / t75;
        let t138 = t28 * t137;
        let t142 = t75 * t29;
        let t143 = 1.0 / t142;
        let t144 = t143 * t79;
        let t151 = sigma[ip] * sigma[ip];
        let t152 = t72 * t151;
        let t153 = t75 * t65;
        let t155 = 1.0 / t18 / t153;
        let t156 = t45 * t45;
        let t157 = 1.0 / t156;
        let t159 = t152 * t155 * t157;
        let t161 = t35 * t27;
        let t162 = t50 * t34 * t161;
        let t168 = t138 * t46;
        let t175 = t157 * t34 * t161;
        let t178 = 0.77252592592592592593e0 * t26 * t168 - 0.16152814814814814814e1 * t74 * t144 + 0.7022962962962962963e-1 * t152 * t155 * t175;
        let t179 = t46 * t178;
        let t183 = 11.0 / 27.0 * t26 * t138 * t51 - 23.0 / 27.0 * t74 * t144 * t50 - 2.0 / 9.0 * t26 * t68 * t90 + t159 * t162 / 27.0 + 2.0 / 9.0 * t74 * t80 * t89 + t26 * t33 * t179 / 24.0;
        let t188 = piecewise3(t2, 0.0, t6 * t129 * t55 / 12.0 - t6 * t61 * t94 / 4.0 - 3.0 / 8.0 * t6 * t19 * t183);
        let tv2rho20 = 2.0 * rho[ip] * t188 + 4.0 * t99;
        v2rho2[ip] += tv2rho20;
        let t194 = t67 * t46;
        let t198 = t72 * t77;
        let t199 = t79 * t50;
        let t200 = t199 * t37;
        let t206 = t72 * sigma[ip];
        let t208 = 1.0 / t18 / t142;
        let t210 = t206 * t208 * t157;
        let t224 = t79 * t37;
        let t230 = -0.21068888888888888889e0 * t25 * t84 + 0.52672222222222222222e0 * t198 * t224 - 0.26336111111111111111e-1 * t206 * t208 * t175;
        let t231 = t46 * t230;
        let t235 = -t102 * t194 * t50 / 9.0 + 5.0 / 18.0 * t198 * t200 + t102 * t103 * t89 / 24.0 - t210 * t162 / 72.0 - t106 * t108 * t89 / 24.0 - t26 * t68 * t116 / 9.0 + t74 * t80 * t115 / 9.0 + t26 * t33 * t231 / 24.0;
        let t240 = piecewise3(t2, 0.0, -t6 * t61 * t120 / 8.0 - 3.0 / 8.0 * t6 * t19 * t235);
        let tv2rhosigma0 = 2.0 * rho[ip] * t240 + 2.0 * t124;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = t72 * t107;
        let t244 = 1.0 / t37;
        let t245 = t199 * t244;
        let t253 = t72 / t18 / t76;
        let t254 = t253 * t157;
        let t260 = t79 * t244;
        let t263 = t36 * t27;
        let t266 = -0.1185125e0 * t243 * t260 + 0.98760416666666666666e-2 * t254 * t263;
        let t267 = t46 * t266;
        let t271 = -t243 * t245 / 16.0 + t102 * t103 * t115 / 12.0 + t254 * t162 / 192.0 - t106 * t108 * t115 / 12.0 + t26 * t33 * t267 / 24.0;
        let t275 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t271);
        let tv2sigma20 = 2.0 * rho[ip] * t275;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (72 lines) ---
        let t278 = t17 * t32;
        let t289 = 1.0 / t30 / t76;
        let t290 = t28 * t289;
        let t294 = 1.0 / t153;
        let t295 = t294 * t79;
        let t302 = t75 * t75;
        let t304 = 1.0 / t18 / t302;
        let t306 = t152 * t304 * t157;
        let t315 = t37 * t151;
        let t316 = t72 * t315;
        let t317 = t302 * rho[ip];
        let t319 = 1.0 / t30 / t317;
        let t321 = 1.0 / t156 / t44;
        let t323 = t316 * t319 * t321;
        let t325 = t24 * t28;
        let t326 = t50 * t20 * t325;
        let t330 = t89 * t34 * t161;
        let t336 = t290 * t46;
        let t346 = t321 * t20 * t325;
        let t349 = -0.36051209876543209877e1 * t26 * t336 + 0.11751758024691358024e2 * t74 * t295 - 0.10534444444444444444e1 * t152 * t304 * t175 + 0.18727901234567901235e0 * t316 * t319 * t346;
        let t350 = t46 * t349;
        let t354 = -154.0 / 81.0 * t26 * t290 * t51 + 502.0 / 81.0 * t74 * t295 * t50 + 11.0 / 9.0 * t26 * t138 * t90 - 5.0 / 9.0 * t306 * t162 - 23.0 / 9.0 * t74 * t144 * t89 - t26 * t68 * t179 / 3.0 + 8.0 / 81.0 * t323 * t326 + t159 * t330 / 9.0 + t74 * t80 * t178 / 3.0 + t26 * t33 * t350 / 24.0;
        let t359 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t278 * t55 + t6 * t129 * t94 / 4.0 - 3.0 / 8.0 * t6 * t61 * t183 - 3.0 / 8.0 * t6 * t19 * t354);
        let tv3rho30 = 2.0 * rho[ip] * t359 + 6.0 * t188;
        v3rho3[ip] += tv3rho30;
        let t369 = t137 * t46;
        let t373 = t72 * t143;
        let t379 = t72 * t155;
        let t380 = t157 * t50;
        let t383 = sigma[ip] * t34 * t161;
        let t386 = t79 * t89;
        let t387 = t386 * t37;
        let t394 = 1.0 / t30 / t302;
        let t396 = t74 * t394 * t321;
        let t414 = t115 * t34 * t161;
        let t424 = t379 * t157;
        let t430 = 0.77252592592592592593e0 * t25 * t168 - 0.31954481481481481481e1 * t373 * t224 + 0.34236944444444444444e0 * t424 * t383 - 0.70229629629629629629e-1 * t74 * t394 * t346;
        let t431 = t46 * t430;
        let t435 = 11.0 / 27.0 * t102 * t369 * t50 - 91.0 / 54.0 * t373 * t200 - 2.0 / 9.0 * t102 * t194 * t89 + 13.0 / 72.0 * t379 * t380 * t383 + 5.0 / 9.0 * t198 * t387 + t102 * t103 * t178 / 24.0 - t396 * t326 / 27.0 - t210 * t330 / 36.0 - t106 * t108 * t178 / 24.0 + 11.0 / 27.0 * t26 * t138 * t116 - 23.0 / 27.0 * t74 * t144 * t115 - 2.0 / 9.0 * t26 * t68 * t231 + t159 * t414 / 27.0 + 2.0 / 9.0 * t74 * t80 * t230 + t26 * t33 * t431 / 24.0;
        let t440 = piecewise3(t2, 0.0, t6 * t129 * t120 / 12.0 - t6 * t61 * t235 / 4.0 - 3.0 / 8.0 * t6 * t19 * t435);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t440 + 4.0 * t240;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t448 = t72 * t208;
        let t449 = t448 * t157;
        let t452 = t386 * t244;
        let t458 = t79 * t115;
        let t459 = t458 * t37;
        let t467 = t72 / t30 / t153;
        let t468 = t321 * t50;
        let t469 = t467 * t468;
        let t471 = t25 * t28 * t37;
        let t491 = t467 * t321;
        let t494 = 0.47405e0 * t198 * t260 - 0.92176388888888888889e-1 * t449 * t263 + 0.26336111111111111111e-1 * t491 * t471;
        let t495 = t46 * t494;
        let t499 = t198 * t245 / 4.0 - 7.0 / 144.0 * t449 * t162 - t243 * t452 / 16.0 - 2.0 / 9.0 * t102 * t194 * t115 + 5.0 / 9.0 * t198 * t459 + t102 * t103 * t230 / 12.0 + t469 * t471 / 72.0 + t254 * t330 / 192.0 - t210 * t414 / 36.0 - t106 * t108 * t230 / 12.0 - t26 * t68 * t267 / 9.0 + t74 * t80 * t266 / 9.0 + t26 * t33 * t495 / 24.0;
        let t504 = piecewise3(t2, 0.0, -t6 * t61 * t271 / 8.0 - 3.0 / 8.0 * t6 * t19 * t499);
        let tv3rhosigma20 = 2.0 * rho[ip] * t504 + 2.0 * t275;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t507 = t253 * t380;
        let t508 = 1.0 / sigma[ip];
        let t510 = t508 * t34 * t161;
        let t513 = t458 * t244;
        let t516 = 1.0 / t73;
        let t517 = t199 * t516;
        let t524 = 1.0 / t30 / t142;
        let t525 = t72 * t524;
        let t526 = t525 * t468;
        let t528 = t25 * t28 * t244;
        let t538 = t79 * t516;
        let t541 = t525 * t321;
        let t544 = 0.148140625e-1 * t254 * t510 + 0.5925625e-1 * t243 * t538 - 0.98760416666666666666e-2 * t541 * t528;
        let t545 = t46 * t544;
        let t549 = t507 * t510 / 128.0 - 3.0 / 16.0 * t243 * t513 + t243 * t517 / 32.0 + t102 * t103 * t266 / 8.0 - t526 * t528 / 192.0 + t254 * t414 / 64.0 - t106 * t108 * t266 / 8.0 + t26 * t33 * t545 / 24.0;
        let t553 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t549);
        let tv3sigma30 = 2.0 * rho[ip] * t553;
        v3sigma3[ip] += tv3sigma30;
        // --- lxc delta (this level) (54 lines) ---
        let t570 = t89 * t20 * t325;
        let t574 = t178 * t34 * t161;
        let t578 = 1.0 / t18 / t317;
        let t585 = t302 * t29;
        let t587 = 1.0 / t30 / t585;
        let t592 = t28 * t524;
        let t596 = 1.0 / t302;
        let t597 = t596 * t79;
        let t606 = t21 * t21;
        let t607 = 1.0 / t606;
        let t609 = t607 * t151 * sigma[ip];
        let t613 = 1.0 / t156 / t45;
        let t614 = 1.0 / t302 / t75 * t613;
        let t649 = 32.0 / 81.0 * t323 * t570 + 2.0 / 9.0 * t159 * t574 + 1627.0 / 243.0 * t152 * t578 * t157 * t162 - 20.0 / 9.0 * t306 * t330 - 592.0 / 243.0 * t316 * t587 * t321 * t326 + t26 * t33 * t46 * (0.20429018930041152264e2 * t26 * t592 * t46 - 0.91875962139917695468e2 * t74 * t597 + 0.12695956378600823045e2 * t152 * t578 * t175 - 0.46195489711934156378e1 * t316 * t587 * t346 + 0.12485267489711934157e1 * t609 * t614) / 24.0 + 22.0 / 9.0 * t26 * t138 * t179 - 4.0 / 9.0 * t26 * t68 * t350 - 616.0 / 81.0 * t26 * t290 * t90 + 160.0 / 243.0 * t609 * t614 * t50 + 4.0 / 9.0 * t74 * t80 * t349 - 11774.0 / 243.0 * t74 * t597 * t50 + 2008.0 / 81.0 * t74 * t295 * t89 - 46.0 / 9.0 * t74 * t144 * t178 + 2618.0 / 243.0 * t26 * t592 * t51;
        let t654 = piecewise3(t2, 0.0, 10.0 / 27.0 * t6 * t17 * t67 * t55 - 5.0 / 9.0 * t6 * t278 * t94 + t6 * t129 * t183 / 2.0 - t6 * t61 * t354 / 2.0 - 3.0 / 8.0 * t6 * t19 * t649);
        let tv4rho40 = 2.0 * rho[ip] * t654 + 8.0 * t359;
        v4rho4[ip] += tv4rho40;
        let t669 = t72 * t294;
        let t672 = t72 * t304;
        let t676 = t72 * t319;
        let t679 = t73 * t20 * t325;
        let t682 = t607 * t151;
        let t685 = 1.0 / t302 / t65 * t613;
        let t704 = t230 * t34 * t161;
        let t708 = t115 * t20 * t325;
        let t726 = t26 * t33 * t46 * (-0.36051209876543209877e1 * t25 * t336 + 0.21232758024691358025e2 * t669 * t224 - 0.35758586419753086419e1 * t672 * t157 * t383 + 0.15216419753086419753e1 * t676 * t321 * t679 - 0.46819753086419753086e0 * t682 * t685) / 24.0 + 11.0 / 9.0 * t26 * t138 * t231 - t26 * t68 * t431 / 3.0 - 154.0 / 81.0 * t102 * t289 * t46 * t50 + t159 * t704 / 9.0 + 8.0 / 81.0 * t323 * t708 - 154.0 / 81.0 * t26 * t290 * t116 - 5.0 / 9.0 * t306 * t414 + 65.0 / 81.0 * t676 * t468 * t679 - t396 * t570 / 9.0 - t210 * t574 / 24.0 - 611.0 / 324.0 * t672 * t380 * t383;
        let t727 = t157 * t89;
        let t751 = t79 * t178;
        let t763 = 13.0 / 24.0 * t379 * t727 * t383 - t102 * t194 * t178 / 3.0 + t102 * t103 * t349 / 24.0 + 11.0 / 9.0 * t102 * t369 * t89 + 502.0 / 81.0 * t74 * t295 * t115 - 23.0 / 9.0 * t74 * t144 * t230 + t74 * t80 * t430 / 3.0 - 91.0 / 18.0 * t373 * t387 + 5.0 / 6.0 * t198 * t751 * t37 - 20.0 / 81.0 * t682 * t685 * t50 - t106 * t108 * t349 / 24.0 + 907.0 / 81.0 * t669 * t200;
        let t769 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t278 * t120 + t6 * t129 * t235 / 4.0 - 3.0 / 8.0 * t6 * t61 * t435 - 3.0 / 8.0 * t6 * t19 * (t726 + t763));
        let tv4rho3sigma0 = 2.0 * rho[ip] * t769 + 6.0 * t440;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t786 = t72 * t394;
        let t791 = t607 / t585;
        let t804 = t266 * t34 * t161;
        let t807 = t321 * t89;
        let t818 = t157 * t115;
        let t830 = -2.0 / 9.0 * t26 * t68 * t495 + t26 * t33 * t46 * (-0.237025e1 * t373 * t260 + 0.74180046296296296297e0 * t424 * t263 - 0.44771388888888888888e0 * t786 * t321 * t471 + 0.17557407407407407407e0 * t791 * t613 * sigma[ip]) / 24.0 + 11.0 / 27.0 * t26 * t138 * t267 + t159 * t804 / 27.0 + t467 * t807 * t471 / 36.0 - t210 * t704 / 18.0 - 2.0 / 27.0 * t396 * t708 - 17.0 / 72.0 * t786 * t468 * t471 + 13.0 / 36.0 * t379 * t818 * t383 - 4.0 / 9.0 * t102 * t194 * t230 + t102 * t103 * t430 / 12.0 - 5.0 / 4.0 * t373 * t245;
        let t842 = t79 * t230;
        let t846 = t613 * t50;
        let t864 = -t106 * t108 * t430 / 12.0 - 23.0 / 27.0 * t74 * t144 * t266 + 2.0 / 9.0 * t74 * t80 * t494 - 91.0 / 27.0 * t373 * t459 + 10.0 / 9.0 * t198 * t842 * t37 + 5.0 / 54.0 * t791 * t846 * sigma[ip] + t198 * t452 / 2.0 - t243 * t751 * t244 / 16.0 + t254 * t574 / 192.0 + 169.0 / 432.0 * t424 * t162 - 7.0 / 72.0 * t449 * t330 + 22.0 / 27.0 * t102 * t369 * t115;
        let t870 = piecewise3(t2, 0.0, t6 * t129 * t271 / 12.0 - t6 * t61 * t499 / 4.0 - 3.0 / 8.0 * t6 * t19 * (t830 + t864));
        let tv4rho2sigma20 = 2.0 * rho[ip] * t870 + 4.0 * t504;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t899 = t79 * t266;
        let t907 = t607 / t317;
        let t913 = t321 * t115;
        let t943 = -t448 * t380 * t510 / 32.0 + t469 * t528 / 18.0 + t253 * t727 * t510 / 128.0 + 3.0 / 4.0 * t198 * t513 - 7.0 / 48.0 * t449 * t414 - 3.0 / 16.0 * t243 * t842 * t244 - t198 * t517 / 8.0 + t243 * t386 * t516 / 32.0 - t102 * t194 * t266 / 3.0 + 5.0 / 6.0 * t198 * t899 * t37 + t102 * t103 * t494 / 8.0 - 5.0 / 144.0 * t907 * t846 - t525 * t807 * t528 / 192.0 + t467 * t913 * t471 / 24.0 + t254 * t704 / 64.0 - t210 * t804 / 24.0 - t106 * t108 * t494 / 8.0 - t26 * t68 * t545 / 9.0 + t74 * t80 * t544 / 9.0 + t26 * t33 * t46 * (-0.5925625e-1 * t449 * t510 + 0.10534444444444444444e0 * t491 * t528 - 0.237025e0 * t198 * t538 - 0.65840277777777777777e-1 * t907 * t613) / 24.0;
        let t948 = piecewise3(t2, 0.0, -t6 * t61 * t549 / 8.0 - 3.0 / 8.0 * t6 * t19 * t943);
        let tv4rhosigma30 = 2.0 * rho[ip] * t948 + 2.0 * t553;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t952 = t516 * t20 * t325;
        let t960 = 1.0 / t151 * t34 * t161;
        let t969 = 1.0 / t315;
        let t976 = t607 * t596;
        let t1003 = -t526 * t952 / 192.0 + t253 * t818 * t510 / 32.0 - 3.0 / 256.0 * t507 * t960 - 3.0 / 8.0 * t243 * t899 * t244 + t243 * t458 * t516 / 8.0 - 3.0 / 64.0 * t243 * t199 * t969 + t102 * t103 * t544 / 6.0 + 5.0 / 384.0 * t976 * t846 * t508 - t525 * t913 * t528 / 48.0 + t254 * t804 / 32.0 - t106 * t108 * t544 / 6.0 + t26 * t33 * t46 * (-0.98760416666666666667e-2 * t541 * t952 - 0.2222109375e-1 * t254 * t960 - 0.88884375e-1 * t243 * t79 * t969 + 0.24690104166666666666e-1 * t976 * t613 * t508) / 24.0;
        let t1007 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t1003);
        let tv4sigma40 = 2.0 * rho[ip] * t1007;
        v4sigma4[ip] += tv4sigma40;
    }
}
