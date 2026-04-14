//! GGA_K_PEARSON fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 39 shared lines across all orders.
//! Delta: 29 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_pearson_fxc_unpol(
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
        // --- shared preamble (39 lines) ---
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
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t29 = t24 / t27;
        let t30 = t29 * sigma[ip];
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = rho[ip] * rho[ip];
        let t37 = t25 * t25;
        let t38 = 1.0 / t37;
        let t39 = sigma[ip] * sigma[ip];
        let t40 = t39 * sigma[ip];
        let t41 = t38 * t40;
        let t42 = t33 * t33;
        let t43 = t42 * t42;
        let t47 = 1.0 + t41 / t43 / 576.0;
        let t48 = 1.0 / t47;
        let t49 = t32 / t22 / t33 * t48;
        let t52 = 1.0 + 5.0 / 648.0 * t30 * t49;
        let t56 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        // --- vxc delta (18 lines) ---
        let t58 = t20 / t21;
        let t62 = t33 * rho[ip];
        let t66 = t32 / t22 / t62 * t48;
        let t69 = t39 * t39;
        let t70 = t29 * t69;
        let t71 = t43 * t62;
        let t73 = 1.0 / t22 / t71;
        let t75 = t47 * t47;
        let t76 = 1.0 / t75;
        let t77 = t76 * t38;
        let t81 = -5.0 / 243.0 * t30 * t66 + 5.0 / 46656.0 * t70 * t32 * t73 * t77;
        let t86 = piecewise3(t2, 0.0, t7 * t58 * t52 / 10.0 + 3.0 / 20.0 * t7 * t23 * t81);
        let tvrho0 = 2.0 * rho[ip] * t86 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t92 = t43 * t33;
        let t94 = 1.0 / t22 / t92;
        let t99 = 5.0 / 648.0 * t29 * t49 - 5.0 / 124416.0 * t29 * t40 * t32 * t94 * t77;
        let t103 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (29 lines) ---
        let t108 = t20 / t21 / rho[ip];
        let t118 = t32 / t22 / t42 * t48;
        let t121 = t43 * t42;
        let t123 = 1.0 / t22 / t121;
        let t128 = t69 * t40;
        let t129 = t29 * t128;
        let t130 = t43 * t43;
        let t133 = 1.0 / t22 / t130 / t42;
        let t136 = 1.0 / t75 / t47;
        let t137 = t37 * t37;
        let t138 = 1.0 / t137;
        let t139 = t136 * t138;
        let t143 = 55.0 / 729.0 * t30 * t118 - 215.0 / 139968.0 * t70 * t32 * t123 * t77 + 5.0 / 1679616.0 * t129 * t32 * t133 * t139;
        let t148 = piecewise3(t2, 0.0, -t7 * t108 * t52 / 30.0 + t7 * t58 * t81 / 5.0 + 3.0 / 20.0 * t7 * t23 * t143);
        let tv2rho20 = 2.0 * rho[ip] * t148 + 4.0 * t86;
        v2rho2[ip] += tv2rho20;
        let t156 = t29 * t32;
        let t157 = t73 * t76;
        let t161 = t69 * t39;
        let t165 = 1.0 / t22 / t130 / t62;
        let t170 = -5.0 / 243.0 * t29 * t66 + 25.0 / 46656.0 * t156 * t157 * t41 - 5.0 / 4478976.0 * t29 * t161 * t32 * t165 * t139;
        let t175 = piecewise3(t2, 0.0, t7 * t58 * t99 / 10.0 + 3.0 / 20.0 * t7 * t23 * t170);
        let tv2rhosigma0 = 2.0 * rho[ip] * t175 + 2.0 * t103;
        v2rhosigma[ip] += tv2rhosigma0;
        let t178 = t94 * t76;
        let t179 = t38 * t39;
        let t183 = t69 * sigma[ip];
        let t187 = 1.0 / t22 / t130 / t33;
        let t192 = -5.0 / 31104.0 * t156 * t178 * t179 + 5.0 / 0.11943936e8 * t29 * t183 * t32 * t187 * t139;
        let t196 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t192);
        let tv2sigma20 = 2.0 * rho[ip] * t196;
        v2sigma2[ip] += tv2sigma20;
    }
}
