//! MGGA_X_2D_PRP10 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_vxc/mgga_x_2d_prp10.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::bessel::{xc_bessel_I0, xc_bessel_I1};
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::lambert_w::{lambert_w};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_2d_prp10_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < vrho.len() {
        let t2 = rho[ip] * rho[ip];
        let t3 = 1.0 / t2;
        let t7 = 2.0 * tau[ip] * t3;
        let t9 = 1.0 / t2 / rho[ip];
        let t11 = sigma[ip] * t9 / 4.0;
        let t13 = 1.0 / M_PI;
        let t14 = (lapl[ip] * t3 / 2.0 - t7 + t11) * t13;
        let t15 = -0.9999999999e0 < t14;
        let t16 = piecewise3::<f64>(t15, t14, -0.9999999999e0);
        let t17 = f64::exp(-1.0);
        let t19 = lambert_w::<f64>(t16 * t17);
        let t20 = t19 + 1.0;
        let t21 = t20 / 2.0;
        let t22 = xc_bessel_I0::<f64>(t21);
        let t24 = t7 - t11;
        let t25 = 0.1e-9 < t24;
        let t26 = piecewise3::<f64>(t25, t24, 0.1e-9);
        let t27 = f64::sqrt(t26);
        let t31 = M_SQRT2;
        let t32 = (M_PI * t22 - 4.0 / 3.0 * t13 * t27) * t31;
        let t33 = f64::sqrt(rho[ip]);
        let tvrho0 = -t32 * t33 / 2.0;
        vrho[ip] += tvrho0;
    }
}
