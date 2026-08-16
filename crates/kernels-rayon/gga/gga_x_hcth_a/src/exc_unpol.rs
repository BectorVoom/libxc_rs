//! GGA_X_HCTH_A exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hcth_a.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hcth_a_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t25 = M_CBRT4;
        let t26 = t20 / t22 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t18 * t18;
        let t33 = 1.0 / t31 / t30;
        let t34 = f64::sqrt(sigma[ip]);
        let t35 = t34 * t27;
        let t37 = 1.0 / t18 / rho[ip];
        let t39 = f64::ln(t35 * t37 + f64::sqrt(pow_2(t35 * t37) + 1.0));
        let t40 = t37 * t39;
        let t43 = 1.0 + 0.252e-1 * t35 * t40;
        let t46 = t43 * t43;
        let t47 = 1.0 / t46;
        let t49 = -0.251173e1 / t43 + 0.37198333333333333333e1 * t47;
        let t54 = 0.109878e1 + 0.93333333333333333332e-3 * t26 * t29 * t33 * t49;
        let t58 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t54);
        let tzk0 = 2.0 * t58;
        zk[ip] += tzk0;
    }
}
