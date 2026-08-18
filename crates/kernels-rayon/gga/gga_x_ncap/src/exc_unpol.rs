//! GGA_X_NCAP exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ncap_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
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
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t31 = t28 * t30;
        let t33 = t25 * t31 / 12.0;
        let t34 = f64::tanh(t33);
        let t35 = param_mu * t34;
        let t36 = f64::ln(t33 + f64::sqrt(t33 * t33 + 1.0));
        let t37 = 1.0 - param_zeta;
        let t39 = t37 * t21 * t24;
        let t40 = 1.0 + t33;
        let t41 = f64::ln(t40);
        let t42 = t30 * t41;
        let t46 = param_zeta * t21 * t24;
        let t51 = 1.0 + param_alpha * (t39 * t28 * t42 / 12.0 + t46 * t31 / 12.0);
        let t52 = t36 * t51;
        let t53 = param_beta * t34;
        let t55 = t53 * t36 + 1.0;
        let t56 = 1.0 / t55;
        let t57 = t52 * t56;
        let t59 = t35 * t57 + 1.0;
        let t63 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t59);
        let tzk0 = 2.0 * t63;
        zk[ip] += tzk0;
    }
}
