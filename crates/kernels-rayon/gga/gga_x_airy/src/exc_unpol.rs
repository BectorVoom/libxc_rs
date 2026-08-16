//! GGA_X_AIRY exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_airy_exc_unpol(
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
        let t32 = t25 * t28 * t30;
        let t33 = f64::powf(t32, 0.2626712e1);
        let t35 = 1.0 + 0.13471619689594796103e-3 * t33;
        let t36 = f64::powf(t35, -0.657946e0);
        let t39 = f64::powf(t32, 0.3217063e1);
        let t41 = f64::powf(t32, 0.3223476e1);
        let t43 = 1.0 - 0.45212413010769857073e-1 * t39 + 0.45402221956620378581e-1 * t41;
        let t44 = f64::powf(t32, 0.3473804e1);
        let t46 = 1.0 + 0.47702180224903349918e-3 * t44;
        let t47 = 1.0 / t46;
        let t49 = 0.60146019220211109872e-4 * t33 * t36 + t43 * t47;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
    }
}
