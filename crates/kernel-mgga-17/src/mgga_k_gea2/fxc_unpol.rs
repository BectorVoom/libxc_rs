//! MGGA_K_GEA2 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 32 shared lines across all orders.
//! Delta: 18 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_gea2_fxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (32 lines) ---
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
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = lapl[ip] * t32;
        let t46 = 1.0 + 5.0 / 648.0 * t30 * t33 * t36 + 5.0 / 54.0 * t30 * t40 / t23 / rho[ip];
        let t50 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t46);
        let tzk0 = 2.0 * t50;
        zk[ip] += tzk0;
        // --- vxc delta (14 lines) ---
        let t52 = t21 / t22;
        let t56 = t34 * rho[ip];
        let t58 = 1.0 / t23 / t56;
        let t65 = -5.0 / 243.0 * t30 * t33 * t58 - 25.0 / 162.0 * t30 * t40 * t36;
        let t70 = piecewise3(t3, 0.0, t8 * t52 * t46 / 10.0 + 3.0 / 20.0 * t8 * t24 * t65);
        let tvrho0 = 2.0 * rho[ip] * t70 + 2.0 * t50;
        vrho[ip] += tvrho0;
        let t73 = t8 * t21;
        let t76 = t29 * t32;
        let t78 = t73 / t34 * t25 * t76;
        let t80 = piecewise3(t3, 0.0, t78 / 864.0);
        let tvsigma0 = 2.0 * rho[ip] * t80;
        vsigma[ip] += tvsigma0;
        let t87 = piecewise3(t3, 0.0, t73 / rho[ip] * t25 * t76 / 72.0);
        let tvlapl0 = 2.0 * rho[ip] * t87;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        // --- fxc delta (this level) (18 lines) ---
        let t92 = t21 / t22 / rho[ip];
        let t99 = t34 * t34;
        let t101 = 1.0 / t23 / t99;
        let t108 = 55.0 / 729.0 * t30 * t33 * t101 + 100.0 / 243.0 * t30 * t40 * t58;
        let t113 = piecewise3(t3, 0.0, -t8 * t92 * t46 / 30.0 + t8 * t52 * t65 / 5.0 + 3.0 / 20.0 * t8 * t24 * t108);
        let tv2rho20 = 2.0 * rho[ip] * t113 + 4.0 * t70;
        v2rho2[ip] += tv2rho20;
        let t119 = t73 / t56 * t25 * t76;
        let t121 = piecewise3(t3, 0.0, -t119 / 432.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t121 + 2.0 * t80;
        v2rhosigma[ip] += tv2rhosigma0;
        let t125 = piecewise3(t3, 0.0, -t78 / 72.0);
        let tv2rholapl0 = 2.0 * rho[ip] * t125 + 2.0 * t87;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
