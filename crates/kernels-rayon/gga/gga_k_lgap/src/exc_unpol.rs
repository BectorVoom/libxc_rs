//! GGA_K_LGAP exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t25 = M_CBRT6;
        let t26 = t25 * t25;
        let t28 = M_PI * M_PI;
        let t29 = pow_1_3(t28);
        let t31 = param_mu_0 * t26 / t29;
        let t32 = f64::sqrt(sigma[ip]);
        let t33 = M_CBRT2;
        let t34 = t32 * t33;
        let t36 = 1.0 / t21 / rho[ip];
        let t41 = param_mu_1 * t25;
        let t42 = t29 * t29;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = t33 * t33;
        let t46 = sigma[ip] * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t55 = param_mu_2 / t28;
        let t56 = t32 * sigma[ip];
        let t57 = t47 * t47;
        let t58 = 1.0 / t57;
        let t63 = f64::exp(-t31 * t34 * t36 / 12.0 - t44 * t46 * t49 / 24.0 - t55 * t56 * t58 / 24.0);
        let t66 = 1.0 + param_kappa * (1.0 - t63);
        let t70 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
    }
}
