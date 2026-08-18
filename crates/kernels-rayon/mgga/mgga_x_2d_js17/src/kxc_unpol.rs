//! MGGA_X_2D_JS17 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_2d_js17_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rho2lapl: &mut [f64],
    v3rho2tau: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3rhosigmalapl: &mut [f64],
    v3rhosigmatau: &mut [f64],
    v3rholapl2: &mut [f64],
    v3rholapltau: &mut [f64],
    v3rhotau2: &mut [f64],
    v3sigma3: &mut [f64],
    v3sigma2lapl: &mut [f64],
    v3sigma2tau: &mut [f64],
    v3sigmalapl2: &mut [f64],
    v3sigmalapltau: &mut [f64],
    v3sigmatau2: &mut [f64],
    v3lapl3: &mut [f64],
    v3lapl2tau: &mut [f64],
    v3lapltau2: &mut [f64],
    v3tau3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = f64::sqrt(M_PI);
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = f64::sqrt(zeta_threshold);
        let t14 = f64::sqrt(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = M_SQRT2;
        let t19 = f64::sqrt(rho[ip]);
        let t20 = t18 * t19;
        let t21 = rho[ip] * rho[ip];
        let t22 = t21 * rho[ip];
        let t23 = 1.0 / t22;
        let t24 = sigma[ip] * t23;
        let t26 = sigma[ip] * sigma[ip];
        let t27 = t21 * t21;
        let t29 = 1.0 / t27 / t21;
        let t32 = 1.0 + 0.8250592249883855 * t24 + 0.0025211952768090192 * t26 * t29;
        let t33 = f64::powf(t32, 1.0 / 15.0);
        let t43 = 1.0 + 0.05587702687752028 * t24 + (-0.1544 * tau[ip] / t21 - 11.596246802930645) / M_PI / 4.0;
        let t44 = f64::powf(t32, 1.0 / 5.0);
        let t45 = 1.0 / t44;
        let t48 = 1.0 / t33 + 2.0 / 5.0 * t43 * t45;
        let t52 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t48);
        let tzk0 = 2.0 * t52;
        zk[ip] += tzk0;
        let t54 = t18 / t19;
        let t59 = 1.0 / t33 / t32;
        let t60 = 1.0 / t27;
        let t61 = sigma[ip] * t60;
        let t63 = t27 * t22;
        let t64 = 1.0 / t63;
        let t67 = -2.475177674965156 * t61 - 0.015127171660854116 * t26 * t64;
        let t73 = -0.16763108063256085 * t61 + 0.02457352321338864 * tau[ip] * t23;
        let t77 = 1.0 / t44 / t32;
        let t78 = t43 * t77;
        let t81 = -t59 * t67 / 15.0 + 2.0 / 5.0 * t73 * t45 - 2.0 / 25.0 * t78 * t67;
        let t86 = piecewise3(t3, 0.0, -t17 * t54 * t48 / 3.0 - 2.0 / 3.0 * t17 * t20 * t81);
        let tvrho0 = 2.0 * rho[ip] * t86 + 2.0 * t52;
        vrho[ip] += tvrho0;
        let t90 = sigma[ip] * t29;
        let t92 = 0.8250592249883855 * t23 + 0.0050423905536180385 * t90;
        let t99 = -t59 * t92 / 15.0 + 0.022350810751008112 * t23 * t45 - 2.0 / 25.0 * t78 * t92;
        let t103 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t105 = t16 * t18;
        let t107 = 1.0 / t19 / rho[ip];
        let t111 = piecewise3(t3, 0.0, 0.0018485501104083812 * t105 * t107 * t45);
        let tvtau0 = 2.0 * rho[ip] * t111;
        vtau[ip] += tvtau0;
        let t114 = t18 * t107;
        let t121 = t32 * t32;
        let t123 = 1.0 / t33 / t121;
        let t124 = t67 * t67;
        let t128 = 1.0 / t27 / rho[ip];
        let t129 = sigma[ip] * t128;
        let t131 = t27 * t27;
        let t132 = 1.0 / t131;
        let t135 = 9.900710699860625 * t129 + 0.10589020162597881 * t26 * t132;
        let t141 = 0.6705243225302434 * t129 - 0.07372056964016592 * tau[ip] * t60;
        let t144 = t73 * t77;
        let t148 = 1.0 / t44 / t121;
        let t149 = t43 * t148;
        let t154 = 16.0 / 225.0 * t123 * t124 - t59 * t135 / 15.0 + 2.0 / 5.0 * t141 * t45 - 4.0 / 25.0 * t144 * t67 + 12.0 / 125.0 * t149 * t124 - 2.0 / 25.0 * t78 * t135;
        let t159 = piecewise3(t3, 0.0, t17 * t114 * t48 / 6.0 - 2.0 / 3.0 * t17 * t54 * t81 - 2.0 / 3.0 * t17 * t20 * t154);
        let tv2rho20 = 2.0 * rho[ip] * t159 + 4.0 * t86;
        v2rho2[ip] += tv2rho20;
        let t165 = t123 * t92;
        let t169 = sigma[ip] * t64;
        let t171 = -2.475177674965156 * t60 - 0.030254343321708232 * t169;
        let t176 = t23 * t77;
        let t181 = t92 * t67;
        let t186 = 16.0 / 225.0 * t165 * t67 - t59 * t171 / 15.0 - 0.06705243225302433 * t60 * t45 - 0.004470162150201623 * t176 * t67 - 2.0 / 25.0 * t144 * t92 + 12.0 / 125.0 * t149 * t181 - 2.0 / 25.0 * t78 * t171;
        let t191 = piecewise3(t3, 0.0, -t17 * t54 * t99 / 3.0 - 2.0 / 3.0 * t17 * t20 * t186);
        let tv2rhosigma0 = 2.0 * rho[ip] * t191 + 2.0 * t103;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t195 = 1.0 / t19 / t21;
        let t199 = t107 * t77;
        let t204 = piecewise3(t3, 0.0, -0.002772825165612572 * t105 * t195 * t45 - 0.0003697100220816762 * t105 * t199 * t67);
        let tv2rhotau0 = 2.0 * rho[ip] * t204 + 2.0 * t111;
        v2rhotau[ip] += tv2rhotau0;
        let t207 = t92 * t92;
        let t218 = 16.0 / 225.0 * t123 * t207 - 0.00033615937024120254 * t59 * t29 - 0.008940324300403245 * t176 * t92 + 12.0 / 125.0 * t149 * t207 - 0.00040339124428944307 * t78 * t29;
        let t222 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t218);
        let tv2sigma20 = 2.0 * rho[ip] * t222;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t227 = piecewise3(t3, 0.0, -0.0003697100220816762 * t105 * t199 * t92);
        let tv2sigmatau0 = 2.0 * rho[ip] * t227;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t230 = t18 * t195;
        let t239 = t121 * t32;
        let t241 = 1.0 / t33 / t239;
        let t242 = t124 * t67;
        let t245 = t123 * t67;
        let t250 = 1.0 / t131 / rho[ip];
        let t253 = -49.50355349930312 * t90 - 0.8471216130078305 * t26 * t250;
        let t259 = -3.352621612651217 * t90 + 0.2948822785606637 * tau[ip] * t128;
        let t262 = t141 * t77;
        let t265 = t73 * t148;
        let t271 = 1.0 / t44 / t239;
        let t272 = t43 * t271;
        let t275 = t67 * t135;
        let t280 = -496.0 / 3375.0 * t241 * t242 + 16.0 / 75.0 * t245 * t135 - t59 * t253 / 15.0 + 2.0 / 5.0 * t259 * t45 - 6.0 / 25.0 * t262 * t67 + 36.0 / 125.0 * t265 * t124 - 6.0 / 25.0 * t144 * t135 - 132.0 / 625.0 * t272 * t242 + 36.0 / 125.0 * t149 * t275 - 2.0 / 25.0 * t78 * t253;
        let t285 = piecewise3(t3, 0.0, -t17 * t230 * t48 / 4.0 + t17 * t114 * t81 / 2.0 - t17 * t54 * t154 - 2.0 / 3.0 * t17 * t20 * t280);
        let tv3rho30 = 2.0 * rho[ip] * t285 + 6.0 * t159;
        v3rho3[ip] += tv3rho30;
        let t295 = t241 * t92;
        let t298 = t123 * t171;
        let t306 = 9.900710699860625 * t128 + 0.21178040325195763 * sigma[ip] * t132;
        let t311 = t60 * t77;
        let t314 = t23 * t148;
        let t325 = t92 * t124;
        let t328 = t171 * t67;
        let t331 = t92 * t135;
        let t336 = -496.0 / 3375.0 * t295 * t124 + 32.0 / 225.0 * t298 * t67 + 16.0 / 225.0 * t165 * t135 - t59 * t306 / 15.0 + 0.26820972901209733 * t128 * t45 + 0.026820972901209737 * t311 * t67 + 0.0053641945802419475 * t314 * t124 - 0.004470162150201623 * t176 * t135 - 2.0 / 25.0 * t262 * t92 + 24.0 / 125.0 * t265 * t181 - 4.0 / 25.0 * t144 * t171 - 132.0 / 625.0 * t272 * t325 + 24.0 / 125.0 * t149 * t328 + 12.0 / 125.0 * t149 * t331 - 2.0 / 25.0 * t78 * t306;
        let t341 = piecewise3(t3, 0.0, t17 * t114 * t99 / 6.0 - 2.0 / 3.0 * t17 * t54 * t186 - 2.0 / 3.0 * t17 * t20 * t336);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t341 + 4.0 * t191;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t346 = 1.0 / t19 / t22;
        let t350 = t195 * t77;
        let t354 = t107 * t148;
        let t362 = piecewise3(t3, 0.0, 0.00693206291403143 * t105 * t346 * t45 + 0.0011091300662450286 * t105 * t350 * t67 + 0.00044365202649801147 * t105 * t354 * t124 - 0.0003697100220816762 * t105 * t199 * t135);
        let tv3rho2tau0 = 2.0 * rho[ip] * t362 + 4.0 * t204;
        v3rho2tau[ip] += tv3rho2tau0;
        let t368 = t241 * t207;
        let t373 = t123 * t29;
        let t386 = t207 * t67;
        let t389 = t92 * t171;
        let t394 = t29 * t67;
        let t399 = -496.0 / 3375.0 * t368 * t67 + 32.0 / 225.0 * t165 * t171 + 0.0003585699949239494 * t373 * t67 + 0.0020169562214472155 * t59 * t64 + 0.026820972901209737 * t311 * t92 + 0.010728389160483895 * t314 * t181 - 0.008940324300403245 * t176 * t171 + 12.0 / 125.0 * t265 * t207 - 132.0 / 625.0 * t272 * t386 + 24.0 / 125.0 * t149 * t389 - 0.00040339124428944307 * t144 * t29 + 0.0004840694931473317 * t149 * t394 + 0.0024203474657366587 * t78 * t64;
        let t404 = piecewise3(t3, 0.0, -t17 * t54 * t218 / 3.0 - 2.0 / 3.0 * t17 * t20 * t399);
        let tv3rhosigma20 = 2.0 * rho[ip] * t404 + 2.0 * t222;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t410 = t105 * t107;
        let t411 = t148 * t92;
        let t412 = t411 * t67;
        let t419 = piecewise3(t3, 0.0, 0.0005545650331225143 * t105 * t350 * t92 + 0.00044365202649801147 * t410 * t412 - 0.0003697100220816762 * t105 * t199 * t171);
        let tv3rhosigmatau0 = 2.0 * rho[ip] * t419 + 2.0 * t227;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let tv3rholapl20 = 0.0;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let t422 = t207 * t92;
        let t433 = t92 * t29;
        let t436 = -496.0 / 3375.0 * t241 * t422 + 0.0010757099847718483 * t165 * t29 + 0.016092583740725842 * t314 * t207 - 6.762091019795269e-05 * t250 * t77 - 132.0 / 625.0 * t272 * t422 + 0.001452208479441995 * t149 * t433;
        let t440 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t436);
        let tv3sigma30 = 2.0 * rho[ip] * t440;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t446 = 1.0 / t19 / t63;
        let t451 = piecewise3(t3, 0.0, 0.00044365202649801147 * t105 * t354 * t207 - 1.8642223229225607e-06 * t105 * t446 * t77);
        let tv3sigma2tau0 = 2.0 * rho[ip] * t451;
        v3sigma2tau[ip] += tv3sigma2tau0;
        let tv3sigmalapl20 = 0.0;
        v3sigmalapl2[ip] += tv3sigmalapl20;
        let tv3sigmalapltau0 = 0.0;
        v3sigmalapltau[ip] += tv3sigmalapltau0;
        let tv3sigmatau20 = 0.0;
        v3sigmatau2[ip] += tv3sigmatau20;
        let tv3lapl30 = 0.0;
        v3lapl3[ip] += tv3lapl30;
        let tv3lapl2tau0 = 0.0;
        v3lapl2tau[ip] += tv3lapl2tau0;
        let tv3lapltau20 = 0.0;
        v3lapltau2[ip] += tv3lapltau20;
        let tv3tau30 = 0.0;
        v3tau3[ip] += tv3tau30;
    }
}
