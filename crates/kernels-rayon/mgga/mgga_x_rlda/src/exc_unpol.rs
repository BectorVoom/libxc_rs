//! MGGA_X_RLDA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rlda_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_prefactor: f64,
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
        let t18 = pow_1_3(rho[ip]);
        let t21 = pow_1_3(1.0 / M_PI);
        let t22 = 1.0 / t21;
        let t23 = param_prefactor * t22;
        let t24 = M_CBRT4;
        let t25 = M_CBRT2;
        let t26 = t25 * t25;
        let t27 = tau[ip] * t26;
        let t28 = t18 * t18;
        let t30 = 1.0 / t28 / rho[ip];
        let t33 = lapl[ip] * t26;
        let t36 = 2.0 * t27 * t30 - t33 * t30 / 4.0;
        let t39 = t23 * t24 / t36;
        let t42 = piecewise3(t3, 0.0, -15.0 / 16.0 * t17 * t18 * t39);
        let tzk0 = 2.0 * t42;
        zk[ip] += tzk0;
    }
}
