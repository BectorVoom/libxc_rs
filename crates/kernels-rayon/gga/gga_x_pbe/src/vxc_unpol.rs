//! GGA_X_PBE vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbe_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
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
        let t20 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t37 = param_kappa + param_mu * t20 * t25 * sigma[ip] * t28 * t33 / 24.0;
        let t42 = 1.0 + param_kappa * (1.0 - param_kappa / t37);
        let t46 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t42);
        let tzk0 = 2.0 * t46;
        zk[ip] += tzk0;
        let t52 = t30 * rho[ip];
        let t56 = param_kappa * param_kappa;
        let t58 = t6 * t17 / t18 / t52 * t56;
        let t59 = t37 * t37;
        let t61 = 1.0 / t59 * param_mu;
        let t64 = t25 * sigma[ip] * t28;
        let t65 = t61 * t20 * t64;
        let t69 = piecewise3(t2, 0.0, -t6 * t17 / t31 * t42 / 8.0 + t58 * t65 / 24.0);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t46;
        vrho[ip] += tvrho0;
        let t78 = t20 * t25 * t28;
        let t79 = t61 * t78;
        let t82 = piecewise3(t2, 0.0, -t6 * t17 / t18 / t30 * t56 * t79 / 64.0);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
    }
}
