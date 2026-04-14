//! MGGA_X_2D_JS17 kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 28 shared lines across all orders.
//! Delta: 66 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_2d_js17_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rho2lapl: &mut Array<f64>,
    v3rho2tau: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3rhosigmalapl: &mut Array<f64>,
    v3rhosigmatau: &mut Array<f64>,
    v3rholapl2: &mut Array<f64>,
    v3rholapltau: &mut Array<f64>,
    v3rhotau2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    v3sigma2lapl: &mut Array<f64>,
    v3sigma2tau: &mut Array<f64>,
    v3sigmalapl2: &mut Array<f64>,
    v3sigmalapltau: &mut Array<f64>,
    v3sigmatau2: &mut Array<f64>,
    v3lapl3: &mut Array<f64>,
    v3lapl2tau: &mut Array<f64>,
    v3lapltau2: &mut Array<f64>,
    v3tau3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (28 lines) ---
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
        let t32 = 1.0 + 0.82505922498838542062e0 * t24 + 0.25211952768090192343e-2 * t26 * t29;
        let t33 = f64::powf(t32, 1.0 / 15.0);
        let t43 = 1.0 + 0.55877026877520282455e-1 * t24 + (-0.1544e0 * tau[ip] / t21 - 0.11596246802930644802e2) / M_PI / 4.0;
        let t44 = f64::powf(t32, 1.0 / 5.0);
        let t45 = 1.0 / t44;
        let t48 = 1.0 / t33 + 2.0 / 5.0 * t43 * t45;
        let t52 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t48);
        let tzk0 = 2.0 * t52;
        zk[ip] += tzk0;
        // --- vxc delta (23 lines) ---
        let t54 = t18 / t19;
        let t59 = 1.0 / t33 / t32;
        let t60 = 1.0 / t27;
        let t61 = sigma[ip] * t60;
        let t63 = t27 * t22;
        let t64 = 1.0 / t63;
        let t67 = -0.24751776749651562619e1 * t61 - 0.15127171660854115406e-1 * t26 * t64;
        let t73 = -0.16763108063256084736e0 * t61 + 0.24573523213388639842e-1 * tau[ip] * t23;
        let t77 = 1.0 / t44 / t32;
        let t78 = t43 * t77;
        let t81 = -t59 * t67 / 15.0 + 2.0 / 5.0 * t73 * t45 - 2.0 / 25.0 * t78 * t67;
        let t86 = piecewise3(t3, 0.0, -t17 * t54 * t48 / 3.0 - 2.0 / 3.0 * t17 * t20 * t81);
        let tvrho0 = 2.0 * rho[ip] * t86 + 2.0 * t52;
        vrho[ip] += tvrho0;
        let t90 = sigma[ip] * t29;
        let t92 = 0.82505922498838542062e0 * t23 + 0.50423905536180384686e-2 * t90;
        let t99 = -t59 * t92 / 15.0 + 0.22350810751008112982e-1 * t23 * t45 - 2.0 / 25.0 * t78 * t92;
        let t103 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t105 = t16 * t18;
        let t107 = 1.0 / t19 / rho[ip];
        let t111 = piecewise3(t3, 0.0, 0.18485501104083811416e-2 * t105 * t107 * t45);
        let tvtau0 = 2.0 * rho[ip] * t111;
        vtau[ip] += tvtau0;
        // --- fxc delta (39 lines) ---
        let t114 = t18 * t107;
        let t121 = t32 * t32;
        let t123 = 1.0 / t33 / t121;
        let t124 = t67 * t67;
        let t128 = 1.0 / t27 / rho[ip];
        let t129 = sigma[ip] * t128;
        let t131 = t27 * t27;
        let t132 = 1.0 / t131;
        let t135 = 0.99007106998606250476e1 * t129 + 0.10589020162597880784e0 * t26 * t132;
        let t141 = 0.67052432253024338944e0 * t129 - 0.73720569640165919526e-1 * tau[ip] * t60;
        let t144 = t73 * t77;
        let t148 = 1.0 / t44 / t121;
        let t149 = t43 * t148;
        let t154 = 16.0 / 225.0 * t123 * t124 - t59 * t135 / 15.0 + 2.0 / 5.0 * t141 * t45 - 4.0 / 25.0 * t144 * t67 + 12.0 / 125.0 * t149 * t124 - 2.0 / 25.0 * t78 * t135;
        let t159 = piecewise3(t3, 0.0, t17 * t114 * t48 / 6.0 - 2.0 / 3.0 * t17 * t54 * t81 - 2.0 / 3.0 * t17 * t20 * t154);
        let tv2rho20 = 2.0 * rho[ip] * t159 + 4.0 * t86;
        v2rho2[ip] += tv2rho20;
        let t165 = t123 * t92;
        let t169 = sigma[ip] * t64;
        let t171 = -0.24751776749651562619e1 * t60 - 0.30254343321708230812e-1 * t169;
        let t176 = t23 * t77;
        let t181 = t92 * t67;
        let t186 = 16.0 / 225.0 * t165 * t67 - t59 * t171 / 15.0 - 0.67052432253024338946e-1 * t60 * t45 - 0.44701621502016225964e-2 * t176 * t67 - 2.0 / 25.0 * t144 * t92 + 12.0 / 125.0 * t149 * t181 - 2.0 / 25.0 * t78 * t171;
        let t191 = piecewise3(t3, 0.0, -t17 * t54 * t99 / 3.0 - 2.0 / 3.0 * t17 * t20 * t186);
        let tv2rhosigma0 = 2.0 * rho[ip] * t191 + 2.0 * t103;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t195 = 1.0 / t19 / t21;
        let t199 = t107 * t77;
        let t204 = piecewise3(t3, 0.0, -0.27728251656125717124e-2 * t105 * t195 * t45 - 0.36971002208167622832e-3 * t105 * t199 * t67);
        let tv2rhotau0 = 2.0 * rho[ip] * t204 + 2.0 * t111;
        v2rhotau[ip] += tv2rhotau0;
        let t207 = t92 * t92;
        let t218 = 16.0 / 225.0 * t123 * t207 - 0.33615937024120256457e-3 * t59 * t29 - 0.89403243004032451928e-2 * t176 * t92 + 12.0 / 125.0 * t149 * t207 - 0.40339124428944307749e-3 * t78 * t29;
        let t222 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t218);
        let tv2sigma20 = 2.0 * rho[ip] * t222;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t227 = piecewise3(t3, 0.0, -0.36971002208167622832e-3 * t105 * t199 * t92);
        let tv2sigmatau0 = 2.0 * rho[ip] * t227;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        // --- kxc delta (this level) (66 lines) ---
        let t230 = t18 * t195;
        let t239 = t121 * t32;
        let t241 = 1.0 / t33 / t239;
        let t242 = t124 * t67;
        let t245 = t123 * t67;
        let t250 = 1.0 / t131 / rho[ip];
        let t253 = -0.49503553499303125238e2 * t90 - 0.84712161300783046272e0 * t26 * t250;
        let t259 = -0.33526216126512169472e1 * t90 + 0.2948822785606636781e0 * tau[ip] * t128;
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
        let t306 = 0.99007106998606250476e1 * t128 + 0.21178040325195761568e0 * sigma[ip] * t132;
        let t311 = t60 * t77;
        let t314 = t23 * t148;
        let t325 = t92 * t124;
        let t328 = t171 * t67;
        let t331 = t92 * t135;
        let t336 = -496.0 / 3375.0 * t295 * t124 + 32.0 / 225.0 * t298 * t67 + 16.0 / 225.0 * t165 * t135 - t59 * t306 / 15.0 + 0.26820972901209735578e0 * t128 * t45 + 0.26820972901209735578e-1 * t311 * t67 + 0.53641945802419471157e-2 * t314 * t124 - 0.44701621502016225964e-2 * t176 * t135 - 2.0 / 25.0 * t262 * t92 + 24.0 / 125.0 * t265 * t181 - 4.0 / 25.0 * t144 * t171 - 132.0 / 625.0 * t272 * t325 + 24.0 / 125.0 * t149 * t328 + 12.0 / 125.0 * t149 * t331 - 2.0 / 25.0 * t78 * t306;
        let t341 = piecewise3(t3, 0.0, t17 * t114 * t99 / 6.0 - 2.0 / 3.0 * t17 * t54 * t186 - 2.0 / 3.0 * t17 * t20 * t336);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t341 + 4.0 * t191;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rho2lapl0 = 0.0;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let t346 = 1.0 / t19 / t22;
        let t350 = t195 * t77;
        let t354 = t107 * t148;
        let t362 = piecewise3(t3, 0.0, 0.6932062914031429281e-2 * t105 * t346 * t45 + 0.1109130066245028685e-2 * t105 * t350 * t67 + 0.44365202649801147398e-3 * t105 * t354 * t124 - 0.36971002208167622832e-3 * t105 * t199 * t135);
        let tv3rho2tau0 = 2.0 * rho[ip] * t362 + 4.0 * t204;
        v3rho2tau[ip] += tv3rho2tau0;
        let t368 = t241 * t207;
        let t373 = t123 * t29;
        let t386 = t207 * t67;
        let t389 = t92 * t171;
        let t394 = t29 * t67;
        let t399 = -496.0 / 3375.0 * t368 * t67 + 32.0 / 225.0 * t165 * t171 + 0.35856999492394940221e-3 * t373 * t67 + 0.20169562214472153874e-2 * t59 * t64 + 0.26820972901209735578e-1 * t311 * t92 + 0.10728389160483894231e-1 * t314 * t181 - 0.89403243004032451928e-2 * t176 * t171 + 12.0 / 125.0 * t265 * t207 - 132.0 / 625.0 * t272 * t386 + 24.0 / 125.0 * t149 * t389 - 0.40339124428944307749e-3 * t144 * t29 + 0.48406949314733169299e-3 * t149 * t394 + 0.24203474657366584649e-2 * t78 * t64;
        let t404 = piecewise3(t3, 0.0, -t17 * t54 * t218 / 3.0 - 2.0 / 3.0 * t17 * t20 * t399);
        let tv3rhosigma20 = 2.0 * rho[ip] * t404 + 2.0 * t222;
        v3rhosigma2[ip] += tv3rhosigma20;
        let tv3rhosigmalapl0 = 0.0;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let t410 = t105 * t107;
        let t411 = t148 * t92;
        let t412 = t411 * t67;
        let t419 = piecewise3(t3, 0.0, 0.55456503312251434248e-3 * t105 * t350 * t92 + 0.44365202649801147398e-3 * t410 * t412 - 0.36971002208167622832e-3 * t105 * t199 * t171);
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
        let t436 = -496.0 / 3375.0 * t241 * t422 + 0.10757099847718482066e-2 * t165 * t29 + 0.16092583740725841347e-1 * t314 * t207 - 0.67620910197952682985e-4 * t250 * t77 - 132.0 / 625.0 * t272 * t422 + 0.14522084794419950789e-2 * t149 * t433;
        let t440 = piecewise3(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t436);
        let tv3sigma30 = 2.0 * rho[ip] * t440;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let t446 = 1.0 / t19 / t63;
        let t451 = piecewise3(t3, 0.0, 0.44365202649801147398e-3 * t105 * t354 * t207 - 0.1864222322922560624e-5 * t105 * t446 * t77);
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
