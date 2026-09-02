//! GGA_X_LAG vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lag_vxc_unpol(
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
        let t4 = 1.0 <= zeta_threshold;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t4, t5, t4, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t10 = pow_1_3(zeta_threshold);
        let t12 = pow_1_3(t8);
        let t14 = piecewise3(t8 <= zeta_threshold, t10 * zeta_threshold, t12 * t8);
        let t15 = t3 * t14;
        let t16 = pow_1_3(rho[ip]);
        let t17 = M_CBRT6;
        let t18 = t17 * t17;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3(t19);
        let t21 = 1.0 / t20;
        let t22 = t18 * t21;
        let t23 = rmath::sqrt(sigma[ip]);
        let t24 = M_CBRT2;
        let t29 = t22 * t23 * t24 / t16 / rho[ip];
        let t30 = rmath::pow(t29, 2.626712);
        let t33 = 1.0 + 0.00013471619689594795 * t30;
        let t34 = rmath::pow(t33, -0.657946);
        let t38 = piecewise3(t2, 0.0, -1.540002877192757e-05 * t15 * t16 * t30 * t34);
        let tzk0 = 2.0 * t38;
        zk[ip] += tzk0;
        let t39 = t16 * t16;
        let t45 = rho[ip] * rho[ip];
        let t46 = 1.0 / t45;
        let t47 = rmath::pow(t29, 1.626712);
        let t49 = t15 * t46 * t47;
        let t50 = t34 * t18;
        let t52 = t21 * t23 * t24;
        let t53 = t50 * t52;
        let t56 = rmath::pow(t29, 4.253424);
        let t58 = t15 * t46 * t56;
        let t59 = rmath::pow(t33, -1.657946);
        let t60 = t59 * t18;
        let t61 = t60 * t52;
        let t65 = piecewise3(t2, 0.0, -5.133342923975857e-06 * t15 / t39 * t30 * t34 + 5.393525383408988e-05 * t49 * t53 - 4.780604235623332e-09 * t58 * t61);
        let tvrho0 = 2.0 * rho[ip] * t65 + 2.0 * t38;
        vrho[ip] += tvrho0;
        let t68 = 1.0 / rho[ip];
        let t70 = t15 * t68 * t47;
        let t71 = 1.0 / t23;
        let t73 = t21 * t71 * t24;
        let t74 = t50 * t73;
        let t78 = t15 * t68 * t56;
        let t79 = t60 * t73;
        let t83 = piecewise3(t2, 0.0, -2.0225720187783704e-05 * t70 * t74 + 1.7927265883587494e-09 * t78 * t79);
        let tvsigma0 = 2.0 * rho[ip] * t83;
        vsigma[ip] += tvsigma0;
    }
}
