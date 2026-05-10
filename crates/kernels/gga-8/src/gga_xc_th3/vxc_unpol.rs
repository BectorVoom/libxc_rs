//! GGA_XC_TH3 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 54 shared lines across all orders.
//! Delta: 20 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_xc_th3_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_18: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (54 lines) ---
        let t2 = f64::powf(2.0, 1.0 / 6.0);
        let t3 = t2 * t2;
        let t4 = t3 * t3;
        let t6 = param_omega_0 * t4 * t2;
        let t7 = f64::powf(rho[ip], 1.0 / 6.0);
        let t8 = t7 * rho[ip];
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = param_omega_1 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = t15 * rho[ip];
        let t20 = M_SQRT2;
        let t21 = param_omega_2 * t20;
        let t22 = f64::sqrt(rho[ip]);
        let t23 = t22 * rho[ip];
        let t27 = param_omega_3 * t12;
        let t28 = t15 * t15;
        let t29 = t28 * rho[ip];
        let t33 = f64::powf(2.0, 1.0 / 12.0);
        let t34 = t33 * t33;
        let t36 = t34 * t34;
        let t38 = param_omega_4 * t36 * t34 * t33;
        let t39 = f64::powf(rho[ip], 1.0 / 12.0);
        let t40 = f64::sqrt(sigma[ip]);
        let t43 = pow_1_3(zeta_threshold);
        let t45 = piecewise3(1.0 <= zeta_threshold, t43 * zeta_threshold, 1.0);
        let t50 = param_omega_5 * t20;
        let t56 = param_omega_6 * t12;
        let t62 = param_omega_7 * t2;
        let t68 = param_omega_8 * t12;
        let t69 = 1.0 / rho[ip];
        let t71 = t45 * t45;
        let t76 = param_omega_9 * t2;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = t78 * t7;
        let t80 = 1.0 / t79;
        let t85 = param_omega_10;
        let t86 = 1.0 / t28;
        let t87 = t85 * t86;
        let t88 = sigma[ip] * t71;
        let t92 = param_omega_11 * t12;
        let t93 = rho[ip] * rho[ip];
        let t95 = 1.0 / t28 / t93;
        let t96 = sigma[ip] * t95;
        let t98 = t96 * t71 - t96;
        let t103 = param_omega_12 * t2;
        let t104 = t79 * rho[ip];
        let t108 = param_omega_13;
        let t109 = t108 * t93;
        let t112 = param_omega_18;
        let t113 = f64::powf(rho[ip], 0.10833333333333333333e1);
        let t116 = t6 * t8 / 2.0 + t14 * t16 / 2.0 + t21 * t23 / 2.0 + t27 * t29 / 2.0 + t38 * t39 * t40 * t45 / 4.0 + t50 * t7 * t40 * t45 / 4.0 + t56 * t15 * t40 * t45 / 4.0 + t62 * t22 * t40 * t45 / 4.0 + t68 * t69 * sigma[ip] * t71 / 8.0 + t76 * t80 * sigma[ip] * t71 / 8.0 + t87 * t88 / 8.0 + t92 * t29 * t98 / 2.0 + t103 * t104 * t98 / 2.0 + t109 * t98 / 2.0 + 0.94387431268169349665e0 * t112 * t113;
        let tzk0 = t116 * t69;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (20 lines) ---
        let t125 = t39 * t39;
        let t127 = t125 * t125;
        let t128 = t127 * t127;
        let t129 = t128 * t125 * t39;
        let t130 = 1.0 / t129;
        let t143 = 1.0 / t22;
        let t148 = 1.0 / t93;
        let t153 = 1.0 / t104;
        let t158 = 1.0 / t29;
        let t159 = t85 * t158;
        let t165 = t93 * rho[ip];
        let t167 = 1.0 / t28 / t165;
        let t168 = sigma[ip] * t167;
        let t171 = -8.0 / 3.0 * t168 * t71 + 8.0 / 3.0 * t168;
        let t181 = t108 * rho[ip];
        let t185 = f64::powf(rho[ip], 0.833333333333333333e-1);
        let tvrho0 = 7.0 / 12.0 * t6 * t7 + 2.0 / 3.0 * t14 * t15 + 3.0 / 4.0 * t21 * t22 + 5.0 / 6.0 * t27 * t28 + t38 * t130 * t40 * t45 / 48.0 + t50 * t80 * t40 * t45 / 24.0 + t56 * t86 * t40 * t45 / 12.0 + t62 * t143 * t40 * t45 / 8.0 - t68 * t148 * sigma[ip] * t71 / 8.0 - 5.0 / 48.0 * t76 * t153 * sigma[ip] * t71 - t159 * t88 / 12.0 + 5.0 / 6.0 * t92 * t28 * t98 + t92 * t29 * t171 / 2.0 + 11.0 / 12.0 * t103 * t79 * t98 + t103 * t104 * t171 / 2.0 + t181 * t98 + t109 * t171 / 2.0 + 0.10225305054051679547e1 * t112 * t185;
        vrho[ip] += tvrho0;
        let t188 = 1.0 / t40;
        let t214 = t95 * t71 - t95;
        let tvsigma0 = t38 * t39 * t188 * t45 / 8.0 + t50 * t7 * t188 * t45 / 8.0 + t56 * t15 * t188 * t45 / 8.0 + t62 * t22 * t188 * t45 / 8.0 + t68 * t69 * t71 / 8.0 + t76 * t80 * t71 / 8.0 + t87 * t71 / 8.0 + t92 * t29 * t214 / 2.0 + t103 * t104 * t214 / 2.0 + t109 * t214 / 2.0;
        vsigma[ip] += tvsigma0;
    }
}
