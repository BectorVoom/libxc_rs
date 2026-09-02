//! MGGA_K_GEA4 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_gea4_kxc_unpol(
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
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t30 = t25 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = lapl[ip] * t32;
        let t42 = 1.0 / t23 / rho[ip];
        let t46 = t25 * t25;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = t49 * sigma[ip];
        let t59 = t34 * t34;
        let t61 = 1.0 / t22 / t59;
        let t63 = t31 * t61 * lapl[ip];
        let t66 = sigma[ip] * sigma[ip];
        let t67 = t66 * t31;
        let t68 = t59 * rho[ip];
        let t70 = 1.0 / t22 / t68;
        let t74 = 1.0 + 5.0 / 648.0 * t30 * t33 * t36 + 5.0 / 54.0 * t30 * t40 * t42 + t49 * t51 * t54 / 2916.0 - t58 * t63 / 2592.0 + t49 * t67 * t70 / 8748.0;
        let t78 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t21 / t22;
        let t85 = 1.0 / t23 / t52;
        let t96 = t31 * t70 * lapl[ip];
        let t99 = t59 * t34;
        let t101 = 1.0 / t22 / t99;
        let t105 = -5.0 / 243.0 * t30 * t33 * t85 - 25.0 / 162.0 * t30 * t40 * t36 - 5.0 / 4374.0 * t49 * t51 * t61 + 13.0 / 7776.0 * t58 * t96 - 4.0 / 6561.0 * t49 * t67 * t101;
        let t110 = piecewise3(t3, 0.0, t8 * t80 * t74 / 10.0 + 3.0 / 20.0 * t8 * t24 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t114 = t30 * t32 * t36;
        let t116 = t49 * t63;
        let t118 = sigma[ip] * t31;
        let t120 = t49 * t118 * t70;
        let t122 = 5.0 / 648.0 * t114 - t116 / 2592.0 + t120 / 4374.0;
        let t126 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t122);
        let tvsigma0 = 2.0 * rho[ip] * t126;
        vsigma[ip] += tvsigma0;
        let t138 = 5.0 / 54.0 * t30 * t32 * t42 + t49 * lapl[ip] * t31 * t54 / 1458.0 - t49 * t118 * t61 / 2592.0;
        let t142 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t138);
        let tvlapl0 = 2.0 * rho[ip] * t142;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t147 = t21 / t22 / rho[ip];
        let t155 = 1.0 / t23 / t59;
        let t166 = t31 * t101 * lapl[ip];
        let t171 = 1.0 / t22 / t59 / t52;
        let t175 = 55.0 / 729.0 * t30 * t33 * t155 + 100.0 / 243.0 * t30 * t40 * t85 + 65.0 / 13122.0 * t49 * t51 * t70 - 13.0 / 1458.0 * t58 * t166 + 76.0 / 19683.0 * t49 * t67 * t171;
        let t180 = piecewise3(t3, 0.0, -t8 * t147 * t74 / 30.0 + t8 * t80 * t105 / 5.0 + 3.0 / 20.0 * t8 * t24 * t175);
        let tv2rho20 = 2.0 * rho[ip] * t180 + 4.0 * t110;
        v2rho2[ip] += tv2rho20;
        let t187 = t30 * t32 * t85;
        let t189 = t49 * t96;
        let t192 = t49 * t118 * t101;
        let t194 = -5.0 / 243.0 * t187 + 13.0 / 7776.0 * t189 - 8.0 / 6561.0 * t192;
        let t199 = piecewise3(t3, 0.0, t8 * t80 * t122 / 10.0 + 3.0 / 20.0 * t8 * t24 * t194);
        let tv2rhosigma0 = 2.0 * rho[ip] * t199 + 2.0 * t126;
        v2rhosigma[ip] += tv2rhosigma0;
        let t208 = -25.0 / 162.0 * t114 - 5.0 / 2187.0 * t116 + 13.0 / 7776.0 * t120;
        let t213 = piecewise3(t3, 0.0, t8 * t80 * t138 / 10.0 + 3.0 / 20.0 * t8 * t24 * t208);
        let tv2rholapl0 = 2.0 * rho[ip] * t213 + 2.0 * t142;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let t216 = t8 * t21;
        let t218 = t48 * t31;
        let t220 = t216 * t155 * t46 * t218;
        let t222 = piecewise3(t3, 0.0, t220 / 29160.0);
        let tv2sigma20 = 2.0 * rho[ip] * t222;
        v2sigma2[ip] += tv2sigma20;
        let t226 = t216 * t85 * t46 * t218;
        let t228 = piecewise3(t3, 0.0, -t226 / 17280.0);
        let tv2sigmalapl0 = 2.0 * rho[ip] * t228;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t234 = piecewise3(t3, 0.0, t216 * t36 * t46 * t218 / 9720.0);
        let tv2lapl20 = 2.0 * rho[ip] * t234;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
        let t239 = t21 / t22 / t34;
        let t250 = 1.0 / t23 / t68;
        let t261 = t31 * t171 * lapl[ip];
        let t264 = t59 * t59;
        let t266 = 1.0 / t22 / t264;
        let t270 = -770.0 / 2187.0 * t30 * t33 * t250 - 1100.0 / 729.0 * t30 * t40 * t155 - 520.0 / 19683.0 * t49 * t51 * t101 + 247.0 / 4374.0 * t58 * t261 - 1672.0 / 59049.0 * t49 * t67 * t266;
        let t275 = piecewise3(t3, 0.0, 2.0 / 45.0 * t8 * t239 * t74 - t8 * t147 * t105 / 10.0 + 3.0 / 10.0 * t8 * t80 * t175 + 3.0 / 20.0 * t8 * t24 * t270);
        let tv3rho30 = 2.0 * rho[ip] * t275 + 6.0 * t180;
        v3rho3[ip] += tv3rho30;
        let t286 = t30 * t32 * t155;
        let t288 = t49 * t166;
        let t291 = t49 * t118 * t171;
        let t293 = 55.0 / 729.0 * t286 - 13.0 / 1458.0 * t288 + 152.0 / 19683.0 * t291;
        let t298 = piecewise3(t3, 0.0, -t8 * t147 * t122 / 30.0 + t8 * t80 * t194 / 5.0 + 3.0 / 20.0 * t8 * t24 * t293);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t298 + 4.0 * t199;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t311 = 100.0 / 243.0 * t187 + 65.0 / 6561.0 * t189 - 13.0 / 1458.0 * t192;
        let t316 = piecewise3(t3, 0.0, -t8 * t147 * t138 / 30.0 + t8 * t80 * t208 / 5.0 + 3.0 / 20.0 * t8 * t24 * t311);
        let tv3rho2lapl0 = 2.0 * rho[ip] * t316 + 4.0 * t213;
        v3rho2lapl[ip] += tv3rho2lapl0;
        let tv3rho2tau0 = 0.0;
        v3rho2tau[ip] += tv3rho2tau0;
        let t321 = t216 * t250 * t46 * t218;
        let t323 = piecewise3(t3, 0.0, -7.0 / 43740.0 * t321);
        let tv3rhosigma20 = 2.0 * rho[ip] * t323 + 2.0 * t222;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t327 = piecewise3(t3, 0.0, 11.0 / 51840.0 * t220);
        let tv3rhosigmalapl0 = 2.0 * rho[ip] * t327 + 2.0 * t228;
        v3rhosigmalapl[ip] += tv3rhosigmalapl0;
        let tv3rhosigmatau0 = 0.0;
        v3rhosigmatau[ip] += tv3rhosigmatau0;
        let t331 = piecewise3(t3, 0.0, -t226 / 3645.0);
        let tv3rholapl20 = 2.0 * rho[ip] * t331 + 2.0 * t234;
        v3rholapl2[ip] += tv3rholapl20;
        let tv3rholapltau0 = 0.0;
        v3rholapltau[ip] += tv3rholapltau0;
        let tv3rhotau20 = 0.0;
        v3rhotau2[ip] += tv3rhotau20;
        let tv3sigma30 = 0.0;
        v3sigma3[ip] += tv3sigma30;
        let tv3sigma2lapl0 = 0.0;
        v3sigma2lapl[ip] += tv3sigma2lapl0;
        let tv3sigma2tau0 = 0.0;
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
