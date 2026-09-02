//! MGGA_X_2D_PRHG07 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_prhg07.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::bessel::{xc_bessel_I0};
use libxc_rkernel_math::lambert_w::{lambert_w};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_2d_prhg07_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = rmath::sqrt(zeta_threshold);
        let t12 = rmath::sqrt(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = M_PI * t14;
        let t16 = M_SQRT2;
        let t17 = rmath::sqrt(rho[ip]);
        let t18 = t16 * t17;
        let t19 = rho[ip] * rho[ip];
        let t20 = 1.0 / t19;
        let t25 = t19 * rho[ip];
        let t26 = 1.0 / t25;
        let t30 = 1.0 / M_PI;
        let t31 = (lapl[ip] * t20 / 2.0 - 2.0 * tau[ip] * t20 + sigma[ip] * t26 / 4.0) * t30;
        let t32 = -0.9999999999 < t31;
        let t33 = piecewise3(t32, t31, -0.9999999999);
        let t34 = rmath::exp(-1.0);
        let t36 = lambert_w(t33 * t34);
        let t37 = t36 + 1.0;
        let t38 = t37 / 2.0;
        let t39 = xc_bessel_I0(t38);
        let t43 = piecewise3(t3, 0.0, -t15 * t18 * t39 / 8.0);
        let tzk0 = 2.0 * t43;
        zk[ip] += tzk0;
    }
}
