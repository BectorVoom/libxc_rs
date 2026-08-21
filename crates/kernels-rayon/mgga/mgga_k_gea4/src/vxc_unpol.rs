//! MGGA_K_GEA4 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_k_gea4_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t30 = t25 / t28;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t40 = lapl[ip] * t32;
        let t42 = 1.0 / t23 / rho[ip];
        let t46 = t25 * t25;
        let t48 = 1.0 / t27 / t26;
        let t49 = t46 * t48;
        let t50 = lapl[ip] * lapl[ip];
        let t51 = t50 * t31;
        let t52 = t34 * rho[ip];
        let t54 = 1.0 / t22 / t52;
        let t58 = t49 * sigma[ip];
        let t59 = t34 * t34;
        let t61 = 1.0 / t22 / t59;
        let t63 = t31 * t61 * lapl[ip];
        let t66 = sigma[ip] * sigma[ip];
        let t67 = t66 * t31;
        let t68 = t59 * rho[ip];
        let t70 = 1.0 / t22 / t68;
        let t74 = 1.0 + 5.0 / 648.0 * t30 * t33 * t36 + 5.0 / 54.0 * t30 * t40 * t42 + t49 * t51 * t54 / 2916.0 - t58 * t63 / 2592.0 + t49 * t67 * t70 / 8748.0;
        let t78 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
        let t80 = t21 / t22;
        let t85 = 1.0 / t23 / t52;
        let t96 = t31 * t70 * lapl[ip];
        let t99 = t59 * t34;
        let t101 = 1.0 / t22 / t99;
        let t105 = -5.0 / 243.0 * t30 * t33 * t85 - 25.0 / 162.0 * t30 * t40 * t36 - 5.0 / 4374.0 * t49 * t51 * t61 + 13.0 / 7776.0 * t58 * t96 - 4.0 / 6561.0 * t49 * t67 * t101;
        let t110 = piecewise3(t3, 0.0, t8 * t80 * t74 / 10.0 + 3.0 / 20.0 * t8 * t24 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t78;
        vrho[ip] += tvrho0;
        let t114 = t30 * t32 * t36;
        let t116 = t49 * t63;
        let t118 = sigma[ip] * t31;
        let t120 = t49 * t118 * t70;
        let t122 = 5.0 / 648.0 * t114 - t116 / 2592.0 + t120 / 4374.0;
        let t126 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t122);
        let tvsigma0 = 2.0 * rho[ip] * t126;
        vsigma[ip] += tvsigma0;
        let t138 = 5.0 / 54.0 * t30 * t32 * t42 + t49 * lapl[ip] * t31 * t54 / 1458.0 - t49 * t118 * t61 / 2592.0;
        let t142 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t138);
        let tvlapl0 = 2.0 * rho[ip] * t142;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
    }
}
