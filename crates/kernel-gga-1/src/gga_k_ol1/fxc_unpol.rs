//! GGA_K_OL1 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 32 shared lines across all orders.
//! Delta: 14 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_ol1_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (32 lines) ---
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
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t29 = 1.0 / t22 / t27;
        let t32 = f64::sqrt(sigma[ip]);
        let t33 = t25 * t32;
        let t35 = 1.0 / t21 / rho[ip];
        let t39 = M_CBRT6;
        let t41 = M_PI * M_PI;
        let t42 = pow_1_3(t41);
        let t43 = t42 * t42;
        let t44 = 1.0 / t43;
        let t47 = 1.0 + 5.0 / 9.0 * (t26 * t29 / 72.0 + 0.677e-2 * t33 * t35) * t39 * t44;
        let t51 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t20 * t22 * t47);
        let tzk0 = 2.0 * t51;
        zk[ip] += tzk0;
        // --- vxc delta (13 lines) ---
        let t52 = 1.0 / t21;
        let t57 = t7 * t20;
        let t58 = t27 * rho[ip];
        let t60 = 1.0 / t22 / t58;
        let t64 = 1.0 / t21 / t27;
        let t67 = -t26 * t60 / 27.0 - 0.90266666666666666666e-2 * t33 * t64;
        let t69 = t39 * t44;
        let t74 = piecewise3(t2, 0.0, t7 * t20 * t52 * t47 / 10.0 + t57 * t22 * t67 * t69 / 12.0);
        let tvrho0 = 2.0 * rho[ip] * t74 + 2.0 * t51;
        vrho[ip] += tvrho0;
        let t80 = t25 / t32;
        let t83 = t25 * t29 / 72.0 + 0.3385e-2 * t80 * t35;
        let t88 = piecewise3(t2, 0.0, t57 * t22 * t83 * t69 / 12.0);
        let tvsigma0 = 2.0 * rho[ip] * t88;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (14 lines) ---
        let t99 = t27 * t27;
        let t101 = 1.0 / t22 / t99;
        let t105 = 1.0 / t21 / t58;
        let t108 = 11.0 / 81.0 * t26 * t101 + 0.21062222222222222222e-1 * t33 * t105;
        let t114 = piecewise3(t2, 0.0, -t7 * t20 * t35 * t47 / 30.0 + t57 * t52 * t67 * t69 / 9.0 + t57 * t22 * t108 * t69 / 12.0);
        let tv2rho20 = 2.0 * rho[ip] * t114 + 4.0 * t74;
        v2rho2[ip] += tv2rho20;
        let t125 = -t25 * t60 / 27.0 - 0.45133333333333333333e-2 * t80 * t64;
        let t131 = piecewise3(t2, 0.0, t57 * t52 * t83 * t69 / 18.0 + t57 * t22 * t125 * t69 / 12.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t88;
        v2rhosigma[ip] += tv2rhosigma0;
        let t134 = t4 * t20;
        let t136 = t134 / t22;
        let t140 = t25 / t32 / sigma[ip] * t69;
        let t143 = piecewise3(t2, 0.0, -0.64895402177010868827e-3 * t136 * t140);
        let tv2sigma20 = 2.0 * rho[ip] * t143;
        v2sigma2[ip] += tv2sigma20;
    }
}
