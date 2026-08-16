//! GGA_X_CHACHIYO vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_chachiyo_vxc_unpol(
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
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = t10 + 1.0;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t4 * t4;
        let t21 = t3 * t20;
        let t22 = rho[ip] * rho[ip];
        let t23 = t19 * t19;
        let t25 = 1.0 / t23 / t22;
        let t29 = M_PI * M_PI;
        let t30 = t3 * t3;
        let t31 = t30 * t4;
        let t32 = f64::sqrt(sigma[ip]);
        let t34 = 1.0 / t19 / rho[ip];
        let t36 = t31 * t32 * t34;
        let t38 = 2.0 / 27.0 * t36 + 1.0;
        let t39 = f64::ln(t38);
        let t41 = 4.0 / 81.0 * t21 * sigma[ip] * t25 + t29 * t39;
        let t44 = 2.0 / 9.0 * t36 + t29;
        let t45 = 1.0 / t44;
        let t46 = 1.0 / t39;
        let t47 = t45 * t46;
        let t51 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t41 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        let t52 = 1.0 / t23;
        let t57 = t22 * rho[ip];
        let t59 = 1.0 / t23 / t57;
        let t64 = t4 * t29 * t30;
        let t66 = 1.0 / t19 / t22;
        let t68 = 1.0 / t38;
        let t72 = -32.0 / 243.0 * t21 * sigma[ip] * t59 - 8.0 / 81.0 * t64 * t32 * t66 * t68;
        let t78 = t17 / t22;
        let t79 = t78 * t41;
        let t80 = t44 * t44;
        let t81 = 1.0 / t80;
        let t82 = t81 * t46;
        let t83 = t82 * t32;
        let t86 = t39 * t39;
        let t87 = 1.0 / t86;
        let t88 = t45 * t87;
        let t90 = t88 * t32 * t68;
        let t94 = piecewise3(t2, 0.0, -t18 * t52 * t41 * t47 / 8.0 - 3.0 / 8.0 * t18 * t19 * t72 * t47 - t79 * t83 / 3.0 - t79 * t90 / 9.0);
        let tvrho0 = 2.0 * rho[ip] * t94 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t99 = 1.0 / t32;
        let t104 = 4.0 / 81.0 * t21 * t25 + t64 * t99 * t34 * t68 / 27.0;
        let t110 = t17 / rho[ip];
        let t111 = t110 * t41;
        let t112 = t82 * t99;
        let t116 = t88 * t99 * t68;
        let t120 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t19 * t104 * t47 + t111 * t112 / 8.0 + t111 * t116 / 24.0);
        let tvsigma0 = 2.0 * rho[ip] * t120;
        vsigma[ip] += tvsigma0;
    }
}
