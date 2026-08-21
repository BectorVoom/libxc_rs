//! MGGA_X_TH fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_th_fxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRTPI;
        let t5 = t4 * t4;
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = pow_1_3(zeta_threshold);
        let t14 = pow_1_3(t10);
        let t16 = piecewise3(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = t5 * t16;
        let t18 = rho[ip] * rho[ip];
        let t19 = 1.0 / tau[ip];
        let t22 = M_CBRT2;
        let t23 = 1.0 / rho[ip];
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = M_CBRT4;
        let t33 = t31 * t32;
        let t34 = t22 * (1.0 + 7.0 / 216.0 * sigma[ip] * t23 * t19) * t33;
        let t37 = piecewise3(t3, 0.0, -27.0 / 160.0 * t17 * t18 * t19 * t34);
        let tzk0 = 2.0 * t37;
        zk[ip] += tzk0;
        let t42 = tau[ip] * tau[ip];
        let t43 = 1.0 / t42;
        let t44 = t17 * t43;
        let t46 = t22 * sigma[ip] * t33;
        let t50 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * rho[ip] * t19 * t34 + 7.0 / 1280.0 * t44 * t46);
        let tvrho0 = 2.0 * rho[ip] * t50 + 2.0 * t37;
        vrho[ip] += tvrho0;
        let t53 = t17 * rho[ip];
        let t58 = piecewise3(t3, 0.0, -7.0 / 1280.0 * t53 * t43 * t22 * t33);
        let tvsigma0 = 2.0 * rho[ip] * t58;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t65 = 1.0 / t42 / tau[ip];
        let t67 = t17 * rho[ip] * t65;
        let t71 = piecewise3(t3, 0.0, 27.0 / 160.0 * t17 * t18 * t43 * t34 + 7.0 / 1280.0 * t67 * t46);
        let tvtau0 = 2.0 * rho[ip] * t71;
        vtau[ip] += tvtau0;
        let t82 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * t19 * t34 + 7.0 / 640.0 * t17 * t23 * t43 * t46);
        let tv2rho20 = 2.0 * rho[ip] * t82 + 4.0 * t50;
        v2rho2[ip] += tv2rho20;
        let t86 = t22 * t31 * t32;
        let t89 = piecewise3(t3, 0.0, -7.0 / 1280.0 * t44 * t86);
        let tv2rhosigma0 = 2.0 * rho[ip] * t89 + 2.0 * t58;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = 0.0;
        v2rholapl[ip] += tv2rholapl0;
        let t96 = piecewise3(t3, 0.0, 27.0 / 80.0 * t17 * rho[ip] * t43 * t34);
        let tv2rhotau0 = 2.0 * rho[ip] * t96 + 2.0 * t71;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let t103 = piecewise3(t3, 0.0, 7.0 / 640.0 * t53 * t65 * t22 * t33);
        let tv2sigmatau0 = 2.0 * rho[ip] * t103;
        v2sigmatau[ip] += tv2sigmatau0;
        let tv2lapl20 = 0.0;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let t109 = t42 * t42;
        let t110 = 1.0 / t109;
        let t112 = t17 * rho[ip] * t110;
        let t116 = piecewise3(t3, 0.0, -27.0 / 80.0 * t17 * t18 * t65 * t34 - 7.0 / 320.0 * t112 * t46);
        let tv2tau20 = 2.0 * rho[ip] * t116;
        v2tau2[ip] += tv2tau20;
    }
}
