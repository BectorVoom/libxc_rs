//! GGA_X_Q1D kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q1d.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_q1d_kxc_unpol(
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t33 = t28 * t32;
        let t34 = t25 * t33;
        let t36 = 0.804e0 + 5.0 / 972.0 * t34;
        let t38 = 0.646416e0 / t36;
        let t40 = t20 * t20;
        let t42 = 1.0 / t22 / t21;
        let t43 = t40 * t42;
        let t44 = sigma[ip] * sigma[ip];
        let t45 = t44 * t26;
        let t46 = t29 * t29;
        let t47 = t46 * rho[ip];
        let t49 = 1.0 / t18 / t47;
        let t52 = t43 * t45 * t49 / 288.0;
        let t53 = t34 / 24.0 + t52;
        let t54 = t21 * t21;
        let t55 = 1.0 / t54;
        let t56 = t44 * sigma[ip];
        let t57 = t55 * t56;
        let t58 = t46 * t46;
        let t59 = 1.0 / t58;
        let t62 = 1.0 + t52 + t57 * t59 / 576.0;
        let t63 = 1.0 / t62;
        let t64 = t53 * t63;
        let t66 = (0.1804e1 - t38) * t20;
        let t67 = t66 * t24;
        let t70 = -t67 * t33 / 24.0 + 0.6525e-1;
        let t72 = 0.1804e1 - t38 + t64 * t70;
        let t76 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t72);
        let tzk0 = 2.0 * t76;
        zk[ip] += tzk0;
        let t78 = t17 / t30;
        let t82 = t36 * t36;
        let t83 = 1.0 / t82;
        let t84 = t83 * t20;
        let t85 = t84 * t24;
        let t86 = t29 * rho[ip];
        let t88 = 1.0 / t30 / t86;
        let t89 = t28 * t88;
        let t94 = t46 * t29;
        let t96 = 1.0 / t18 / t94;
        let t97 = t45 * t96;
        let t99 = t43 * t97 / 54.0;
        let t100 = -t25 * t89 / 9.0 - t99;
        let t101 = t100 * t63;
        let t103 = t62 * t62;
        let t104 = 1.0 / t103;
        let t105 = t53 * t104;
        let t106 = t58 * rho[ip];
        let t107 = 1.0 / t106;
        let t110 = -t99 - t57 * t107 / 72.0;
        let t111 = t70 * t110;
        let t113 = t83 * t40;
        let t114 = t113 * t42;
        let t119 = 0.7389300411522633745e-3 * t114 * t97 + t67 * t89 / 9.0;
        let t121 = -0.88671604938271604938e-2 * t85 * t89 + t101 * t70 - t105 * t111 + t64 * t119;
        let t126 = piecewise3(t2, 0.0, -t6 * t78 * t72 / 8.0 - 3.0 / 8.0 * t6 * t19 * t121);
        let tvrho0 = 2.0 * rho[ip] * t126 + 2.0 * t76;
        vrho[ip] += tvrho0;
        let t129 = t24 * t27;
        let t130 = t129 * t32;
        let t137 = sigma[ip] * t26 * t49;
        let t139 = t43 * t137 / 144.0;
        let t140 = t25 * t27 * t32 / 24.0 + t139;
        let t141 = t140 * t63;
        let t143 = t55 * t44;
        let t146 = t139 + t143 * t59 / 192.0;
        let t147 = t70 * t146;
        let t153 = -0.27709876543209876543e-3 * t114 * t137 - t66 * t130 / 24.0;
        let t155 = 0.33251851851851851852e-2 * t84 * t130 + t141 * t70 - t105 * t147 + t64 * t153;
        let t159 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t155);
        let tvsigma0 = 2.0 * rho[ip] * t159;
        vsigma[ip] += tvsigma0;
        let t164 = t17 / t30 / rho[ip];
        let t172 = 1.0 / t82 / t36;
        let t173 = t172 * t40;
        let t174 = t173 * t42;
        let t175 = t46 * t86;
        let t177 = 1.0 / t18 / t175;
        let t178 = t45 * t177;
        let t182 = 1.0 / t30 / t46;
        let t183 = t28 * t182;
        let t189 = 19.0 / 162.0 * t43 * t178;
        let t190 = 11.0 / 27.0 * t25 * t183 + t189;
        let t191 = t190 * t63;
        let t193 = t100 * t104;
        let t199 = 1.0 / t103 / t62;
        let t200 = t53 * t199;
        let t201 = t110 * t110;
        let t202 = t70 * t201;
        let t205 = t119 * t110;
        let t208 = t58 * t29;
        let t209 = 1.0 / t208;
        let t212 = t189 + t57 * t209 / 8.0;
        let t213 = t70 * t212;
        let t215 = t172 * t55;
        let t223 = 0.24326914935053938255e-3 * t215 * t56 * t209 - 0.66503703703703703705e-2 * t114 * t178 - 11.0 / 27.0 * t67 * t183;
        let t225 = -0.48653829870107876509e-3 * t174 * t178 + 0.32512921810699588477e-1 * t85 * t183 + t191 * t70 - 2.0 * t193 * t111 + 2.0 * t101 * t119 + 2.0 * t200 * t202 - 2.0 * t105 * t205 - t105 * t213 + t64 * t223;
        let t230 = piecewise3(t2, 0.0, t6 * t164 * t72 / 12.0 - t6 * t78 * t121 / 4.0 - 3.0 / 8.0 * t6 * t19 * t225);
        let tv2rho20 = 2.0 * rho[ip] * t230 + 4.0 * t126;
        v2rho2[ip] += tv2rho20;
        let t236 = t26 * t96;
        let t237 = t236 * sigma[ip];
        let t240 = t129 * t88;
        let t247 = t43 * t237 / 27.0;
        let t248 = -t25 * t27 * t88 / 9.0 - t247;
        let t249 = t248 * t63;
        let t251 = t140 * t104;
        let t255 = t147 * t110;
        let t258 = t119 * t146;
        let t262 = -t247 - t143 * t107 / 24.0;
        let t263 = t70 * t262;
        let t266 = t153 * t110;
        let t275 = -0.91225931006452268454e-4 * t215 * t107 * t44 + 0.22167901234567901235e-2 * t114 * t237 + t66 * t240 / 9.0;
        let t277 = 0.18245186201290453691e-3 * t174 * t237 - 0.88671604938271604939e-2 * t84 * t240 + t249 * t70 - t251 * t111 + t141 * t119 - t193 * t147 + 2.0 * t200 * t255 - t105 * t258 - t105 * t263 + t101 * t153 - t105 * t266 + t64 * t275;
        let t282 = piecewise3(t2, 0.0, -t6 * t78 * t155 / 8.0 - 3.0 / 8.0 * t6 * t19 * t277);
        let tv2rhosigma0 = 2.0 * rho[ip] * t282 + 2.0 * t159;
        v2rhosigma[ip] += tv2rhosigma0;
        let t285 = t42 * t26;
        let t286 = t285 * t49;
        let t289 = t43 * t26;
        let t290 = t49 * t63;
        let t298 = t146 * t146;
        let t299 = t70 * t298;
        let t302 = t153 * t146;
        let t306 = t43 * t26 * t49;
        let t308 = t55 * sigma[ip];
        let t311 = t306 / 144.0 + t308 * t59 / 96.0;
        let t312 = t70 * t311;
        let t319 = 0.3420972412741960067e-4 * t215 * t59 * sigma[ip] - 0.55419753086419753086e-3 * t113 * t286;
        let t321 = -0.68419448254839201342e-4 * t173 * t286 + t289 * t290 * t70 / 144.0 - 2.0 * t251 * t147 + 2.0 * t141 * t153 + 2.0 * t200 * t299 - 2.0 * t105 * t302 - t105 * t312 + t64 * t319;
        let t325 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t321);
        let tv2sigma20 = 2.0 * rho[ip] * t325;
        v2sigma2[ip] += tv2sigma20;
        let t328 = t17 * t32;
        let t338 = t100 * t199;
        let t341 = t103 * t103;
        let t342 = 1.0 / t341;
        let t343 = t53 * t342;
        let t344 = t201 * t110;
        let t345 = t70 * t344;
        let t348 = t111 * t212;
        let t352 = 1.0 / t18 / t58;
        let t353 = t45 * t352;
        let t357 = 1.0 / t30 / t47;
        let t358 = t28 * t357;
        let t361 = t82 * t82;
        let t363 = 1.0 / t361 * t55;
        let t364 = t58 * t86;
        let t365 = 1.0 / t364;
        let t366 = t56 * t365;
        let t369 = t190 * t104;
        let t376 = t119 * t201;
        let t379 = t223 * t110;
        let t382 = t119 * t212;
        let t386 = 209.0 / 243.0 * t43 * t353;
        let t389 = -t386 - 5.0 / 4.0 * t57 * t365;
        let t390 = t70 * t389;
        let t394 = -154.0 / 81.0 * t25 * t358 - t386;
        let t395 = t394 * t63;
        let t401 = t44 * t44;
        let t402 = t363 * t401;
        let t403 = t58 * t47;
        let t405 = 1.0 / t30 / t403;
        let t416 = 0.10011076104960468418e-4 * t402 * t405 * t20 * t129 - 0.46221138376602482685e-2 * t215 * t366 + 0.55994476451760402379e-1 * t114 * t353 + 154.0 / 81.0 * t67 * t358;
        let t418 = 6.0 * t338 * t202 - 6.0 * t343 * t345 + 6.0 * t200 * t348 + 0.5351921285711866416e-2 * t174 * t353 - 0.15172696844993141289e0 * t85 * t358 - 0.24026582651905124202e-3 * t363 * t366 - 3.0 * t369 * t111 - 6.0 * t193 * t205 - 3.0 * t193 * t213 + 6.0 * t200 * t376 - 3.0 * t105 * t379 - 3.0 * t105 * t382 - t105 * t390 + t395 * t70 + 3.0 * t191 * t119 + 3.0 * t101 * t223 + t64 * t416;
        let t423 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t328 * t72 + t6 * t164 * t121 / 4.0 - 3.0 / 8.0 * t6 * t78 * t225 - 3.0 / 8.0 * t6 * t19 * t418);
        let tv3rho30 = 2.0 * rho[ip] * t423 + 6.0 * t230;
        v3rho3[ip] += tv3rho30;
        let t435 = t258 * t110;
        let t438 = t263 * t110;
        let t441 = t147 * t212;
        let t444 = t129 * t182;
        let t447 = t147 * t201;
        let t450 = t26 * t177;
        let t451 = t450 * sigma[ip];
        let t457 = t58 * t46;
        let t459 = 1.0 / t30 / t457;
        let t460 = t363 * t459;
        let t462 = t56 * t20 * t129;
        let t465 = t209 * t44;
        let t472 = -0.37541535393601756565e-5 * t460 * t462 + 0.15508408271096885638e-2 * t215 * t465 - 0.16010150891632373114e-1 * t114 * t451 - 11.0 / 27.0 * t66 * t444;
        let t478 = 19.0 / 81.0 * t43 * t451;
        let t479 = 11.0 / 27.0 * t25 * t27 * t182 + t478;
        let t480 = t479 * t63;
        let t485 = t223 * t146;
        let t487 = 4.0 * t338 * t255 + 4.0 * t200 * t435 + 4.0 * t200 * t438 + 2.0 * t200 * t441 + 0.32512921810699588478e-1 * t84 * t444 - 6.0 * t343 * t447 - 0.16420667581161408322e-2 * t174 * t451 + t191 * t153 + 2.0 * t101 * t275 + t64 * t472 + t480 * t70 + 2.0 * t249 * t119 + t141 * t223 - t105 * t485;
        let t488 = t119 * t262;
        let t493 = t478 + 3.0 / 8.0 * t143 * t209;
        let t494 = t70 * t493;
        let t498 = t275 * t110;
        let t501 = t153 * t212;
        let t503 = t153 * t201;
        let t506 = t140 * t199;
        let t511 = t248 * t104;
        let t522 = -2.0 * t105 * t488 - t105 * t494 - 2.0 * t193 * t266 - 2.0 * t105 * t498 - t105 * t501 + 2.0 * t200 * t503 + 2.0 * t506 * t202 + 0.90099684944644215758e-4 * t363 * t465 - 2.0 * t511 * t111 - 2.0 * t251 * t205 - t251 * t213 - t369 * t147 - 2.0 * t193 * t258 - 2.0 * t193 * t263;
        let t523 = t487 + t522;
        let t528 = piecewise3(t2, 0.0, t6 * t164 * t155 / 12.0 - t6 * t78 * t277 / 4.0 - 3.0 / 8.0 * t6 * t19 * t523);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t528 + 4.0 * t282;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t537 = t96 * t63;
        let t543 = t299 * t110;
        let t546 = t147 * t262;
        let t549 = t302 * t110;
        let t552 = t312 * t110;
        let t555 = t285 * t96;
        let t564 = 1.0 / t30 / t364;
        let t565 = t363 * t564;
        let t567 = t44 * t20 * t129;
        let t570 = t107 * sigma[ip];
        let t575 = 0.14078075772600658712e-5 * t565 * t567 - 0.45612965503226134227e-3 * t215 * t570 + 0.29557201646090534979e-2 * t113 * t555;
        let t577 = t119 * t311;
        let t579 = t289 * t290 * t119 / 144.0 - t289 * t537 * t70 / 27.0 + 4.0 * t506 * t255 - 6.0 * t343 * t543 + 4.0 * t200 * t546 + 4.0 * t200 * t549 + 2.0 * t200 * t552 + 0.36490372402580907382e-3 * t173 * t555 + 2.0 * t249 * t153 + 2.0 * t141 * t275 + t101 * t319 + t64 * t575 - t105 * t577;
        let t584 = -t43 * t236 / 27.0 - t308 * t107 / 12.0;
        let t585 = t70 * t584;
        let t587 = t319 * t110;
        let t601 = t119 * t298;
        let t606 = t275 * t146;
        let t609 = t153 * t262;
        let t613 = t49 * t104;
        let t617 = -t105 * t585 - t105 * t587 - 0.3378738185424158091e-4 * t363 * t570 - 2.0 * t511 * t147 - 2.0 * t251 * t258 - 2.0 * t251 * t263 - 2.0 * t251 * t266 + 2.0 * t338 * t299 + 2.0 * t200 * t601 - 2.0 * t193 * t302 - 2.0 * t105 * t606 - 2.0 * t105 * t609 - t193 * t312 - t289 * t613 * t111 / 144.0;
        let t618 = t579 + t617;
        let t623 = piecewise3(t2, 0.0, -t6 * t78 * t321 / 8.0 - 3.0 / 8.0 * t6 * t19 * t618);
        let tv3rhosigma20 = 2.0 * rho[ip] * t623 + 2.0 * t325;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t629 = t70 * t55;
        let t630 = t629 * t59;
        let t635 = t298 * t146;
        let t636 = t70 * t635;
        let t639 = t147 * t311;
        let t649 = t153 * t298;
        let t652 = t319 * t146;
        let t655 = t153 * t311;
        let t663 = 1.0 / t30 / t208;
        let t664 = t363 * t663;
        let t666 = sigma[ip] * t20 * t129;
        let t671 = -0.5279278414725247017e-6 * t664 * t666 + 0.10262917238225880201e-3 * t215 * t59;
        let t673 = t289 * t290 * t153 / 48.0 - t105 * t630 / 96.0 + 6.0 * t506 * t299 - 6.0 * t343 * t636 + 6.0 * t200 * t639 - t289 * t613 * t147 / 48.0 - 6.0 * t251 * t302 - 3.0 * t251 * t312 + 6.0 * t200 * t649 - 3.0 * t105 * t652 - 3.0 * t105 * t655 + 0.12670268195340592841e-4 * t363 * t59 + 3.0 * t141 * t319 + t64 * t671;
        let t677 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t673);
        let tv3sigma30 = 2.0 * rho[ip] * t677;
        v3sigma3[ip] += tv3sigma30;
    }
}
