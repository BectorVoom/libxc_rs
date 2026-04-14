//! GGA_X_G96 fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 26 shared lines across all orders.
//! Delta: 25 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_g96_fxc_unpol(
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
        // --- shared preamble (26 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = pow_1_3(1.0 / M_PI);
        let t23 = 1.0 / t22;
        let t25 = M_CBRT4;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t31 = t28 / t18 / rho[ip];
        let t32 = f64::sqrt(t31);
        let t33 = t32 * t31;
        let t37 = 1.0 + 2.0 / 1233.0 * t20 * t23 * t25 * t33;
        let t41 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t37);
        let tzk0 = 2.0 * t41;
        zk[ip] += tzk0;
        // --- vxc delta (13 lines) ---
        let t42 = t18 * t18;
        let t48 = t5 * t17;
        let t49 = rho[ip] * rho[ip];
        let t52 = t48 / t49 * t23;
        let t53 = t25 * t32;
        let t54 = t53 * t28;
        let t58 = piecewise3(t2, 0.0, -t6 * t17 / t42 * t37 / 8.0 + t52 * t54 / 274.0);
        let tvrho0 = 2.0 * rho[ip] * t58 + 2.0 * t41;
        vrho[ip] += tvrho0;
        let t63 = t48 / rho[ip] * t23;
        let t64 = 1.0 / t26;
        let t66 = t53 * t64 * t27;
        let t69 = piecewise3(t2, 0.0, -3.0 / 2192.0 * t63 * t66);
        let tvsigma0 = 2.0 * rho[ip] * t69;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (25 lines) ---
        let t78 = t49 * rho[ip];
        let t81 = t48 / t78 * t23;
        let t84 = t49 * t49;
        let t86 = 1.0 / t18 / t84;
        let t88 = t48 * t86 * t23;
        let t89 = 1.0 / t32;
        let t90 = t25 * t89;
        let t91 = t27 * t27;
        let t92 = sigma[ip] * t91;
        let t93 = t90 * t92;
        let t97 = piecewise3(t2, 0.0, t6 * t17 / t42 / rho[ip] * t37 / 12.0 - 5.0 / 822.0 * t81 * t54 - t88 * t93 / 411.0);
        let tv2rho20 = 2.0 * rho[ip] * t97 + 4.0 * t58;
        v2rho2[ip] += tv2rho20;
        let t103 = 1.0 / t18 / t78;
        let t105 = t23 * t25;
        let t107 = t105 * t89 * t91;
        let t111 = piecewise3(t2, 0.0, 3.0 / 2192.0 * t52 * t66 + t48 * t103 * t107 / 1096.0);
        let tv2rhosigma0 = 2.0 * rho[ip] * t111 + 2.0 * t69;
        v2rhosigma[ip] += tv2rhosigma0;
        let t117 = t48 / t18 / t49 * t23;
        let t118 = 1.0 / sigma[ip];
        let t120 = t90 * t118 * t91;
        let t123 = t26 * sigma[ip];
        let t124 = 1.0 / t123;
        let t126 = t53 * t124 * t27;
        let t130 = piecewise3(t2, 0.0, -3.0 / 8768.0 * t117 * t120 + 3.0 / 4384.0 * t63 * t126);
        let tv2sigma20 = 2.0 * rho[ip] * t130;
        v2sigma2[ip] += tv2sigma20;
    }
}
