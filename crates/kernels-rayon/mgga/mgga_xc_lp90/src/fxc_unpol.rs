//! MGGA_XC_LP90 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_lp90.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_lp90_fxc_unpol(
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
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t10 = 1.0 / t4 / rho[ip];
        let t13 = 0.80569 + 0.00037655 * sigma[ip] * t6 - 0.00037655 * lapl[ip] * t10;
        let t14 = 1.0 / t3;
        let t15 = t14 + 0.0040743;
        let t16 = 1.0 / t15;
        let tzk0 = -t13 * t16;
        zk[ip] += tzk0;
        let t18 = t2 * rho[ip];
        let t20 = 1.0 / t4 / t18;
        let t25 = -0.0010041333333333333 * sigma[ip] * t20 + 0.0006275833333333333 * lapl[ip] * t6;
        let t29 = t15 * t15;
        let t30 = 1.0 / t29;
        let tvrho0 = tzk0 - rho[ip] * t25 * t16 - t14 * t13 * t30 / 3.0;
        vrho[ip] += tvrho0;
        let t33 = t10 * t16;
        let tvsigma0 = -0.00037655 * t33;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.00037655 / t4 * t16;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t39 = t13 * t30;
        let t41 = 1.0 / t3 / rho[ip];
        let t44 = t2 * t2;
        let t46 = 1.0 / t4 / t44;
        let t51 = 0.0036818222222222224 * sigma[ip] * t46 - 0.0016735555555555555 * lapl[ip] * t20;
        let t59 = 1.0 / t29 / t15;
        let tv2rho20 = -2.0 * t25 * t16 - 2.0 / 9.0 * t39 * t41 - rho[ip] * t51 * t16 - 2.0 / 3.0 * t14 * t25 * t30 - 2.0 / 9.0 * t10 * t13 * t59;
        v2rho2[ip] += tv2rho20;
        let t62 = t6 * t16;
        let t64 = 1.0 / t18;
        let t65 = t64 * t30;
        let tv2rhosigma0 = 0.0006275833333333333 * t62 - 0.00012551666666666666 * t65;
        v2rhosigma[ip] += tv2rhosigma0;
        let tv2rholapl0 = -0.00025103333333333333 * t33 + 0.00012551666666666666 / t2 * t30;
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
