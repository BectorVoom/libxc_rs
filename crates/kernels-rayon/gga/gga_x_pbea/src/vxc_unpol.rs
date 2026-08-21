//! GGA_X_PBEA vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbea_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t20 = M_CBRT2;
        let t21 = t20 * t20;
        let t23 = rho[ip] * rho[ip];
        let t24 = t18 * t18;
        let t26 = 1.0 / t24 / t23;
        let t29 = 1.0 + 0.008639940809536326 * sigma[ip] * t21 * t26;
        let t30 = rmath::pow(t29, -0.52);
        let t32 = 1.804 - 0.804 * t30;
        let t36 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t42 = t3 * t17;
        let t43 = t23 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t47 = rmath::pow(t29, -1.52);
        let t49 = t47 * sigma[ip] * t21;
        let t53 = piecewise3(t2, 0.0, -t6 * t17 / t24 * t32 / 8.0 + 0.00246634334405953 * t42 * t45 * t49);
        let tvrho0 = 2.0 * rho[ip] * t53 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t62 = piecewise3(t2, 0.0, -0.0009248787540223239 * t42 / t18 / t23 * t47 * t21);
        let tvsigma0 = 2.0 * rho[ip] * t62;
        vsigma[ip] += tvsigma0;
    }
}
