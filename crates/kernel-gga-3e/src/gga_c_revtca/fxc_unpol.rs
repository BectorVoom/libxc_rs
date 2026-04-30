//! GGA_C_REVTCA fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 35 shared lines across all orders.
//! Delta: 47 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_revtca_fxc_unpol(
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
        // --- shared preamble (35 lines) ---
        let t2 = pow_1_3(zeta_threshold);
        let t3 = t2 * t2;
        let t4 = piecewise3(1.0 <= zeta_threshold, t3, 1.0);
        let t5 = t4 * t4;
        let t6 = t5 * t4;
        let t7 = M_CBRT3;
        let t9 = pow_1_3(1.0 / M_PI);
        let t10 = t7 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t18 = 0.488827e1 + 0.79425925e0 * t10 * t12 / t13;
        let t19 = f64::atan(t18);
        let t21 = -0.655868e0 * t19 + 0.897889e0;
        let t22 = t6 * t21;
        let t23 = t7 * t7;
        let t24 = t22 * t23;
        let t25 = 1.0 / t9;
        let t26 = t25 * t11;
        let t27 = M_CBRT6;
        let t28 = t27 * t27;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT2;
        let t34 = f64::sqrt(sigma[ip]);
        let t35 = t33 * t34;
        let t37 = 1.0 / t13 / rho[ip];
        let t39 = t32 * t35 * t37;
        let t40 = f64::powf(t39, 0.23e1);
        let t42 = 1.0 + 0.47121507034422759993e-2 * t40;
        let t43 = 1.0 / t42;
        let t46 = t24 * t26 * t13 * t43;
        let tzk0 = t46 / 3.0;
        zk[ip] += tzk0;
        // --- vxc delta (20 lines) ---
        let t48 = t18 * t18;
        let t49 = t48 + 1.0;
        let t50 = 1.0 / t49;
        let t51 = t6 * t50;
        let t55 = 1.0 / rho[ip] * t6;
        let t57 = t23 * t25;
        let t58 = t57 * t11;
        let t60 = t42 * t42;
        let t61 = 1.0 / t60;
        let t62 = f64::powf(t39, 0.13e1);
        let t63 = t61 * t62;
        let t64 = t63 * t28;
        let t65 = t31 * t33;
        let t66 = t65 * t34;
        let t67 = t64 * t66;
        let tvrho0 = 4.0 / 9.0 * t46 + 0.69457230103866666663e0 * t51 * t43 + 0.48168651635187710217e-2 * t55 * t21 * t58 * t67;
        vrho[ip] += tvrho0;
        let t70 = t22 * t58;
        let t71 = 1.0 / t34;
        let t72 = t65 * t71;
        let tvsigma0 = -0.18063244363195391331e-2 * t70 * t64 * t72;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (47 lines) ---
        let t76 = t50 * t43;
        let t79 = t13 * t13;
        let t85 = rho[ip] * rho[ip];
        let t86 = 1.0 / t85;
        let t91 = t49 * t49;
        let t92 = 1.0 / t91;
        let t93 = t6 * t92;
        let t94 = t43 * t18;
        let t102 = 1.0 / t13 / t85;
        let t107 = t85 * rho[ip];
        let t109 = 1.0 / t13 / t107;
        let t110 = t109 * t6;
        let t112 = t110 * t21 * t58;
        let t114 = 1.0 / t60 / t42;
        let t115 = f64::powf(t39, 0.26e1);
        let t116 = t114 * t115;
        let t117 = t116 * t27;
        let t118 = t30 * t30;
        let t119 = 1.0 / t118;
        let t120 = t33 * t33;
        let t121 = t119 * t120;
        let t122 = t121 * sigma[ip];
        let t123 = t117 * t122;
        let t126 = f64::powf(t39, 0.3e0);
        let t127 = t61 * t126;
        let t128 = t127 * t27;
        let t129 = t128 * t122;
        let tv2rho20 = 0.92609640138488888884e0 * t55 * t76 + 4.0 / 27.0 * t24 * t26 / t79 * t43 + 0.16056217211729236738e-2 * t24 * t26 * t86 * t67 + 0.36778031659583040509e0 * t93 * t94 * t10 * t12 * t37 + 0.20073966722509356988e-1 * t51 * t63 * t32 * t35 * t102 + 0.83527884012674580095e-3 * t112 * t123 - 0.50095397700595218626e-1 * t112 * t129;
        v2rho2[ip] += tv2rho20;
        let t133 = t50 * t61;
        let t134 = t6 * t37 * t133;
        let t135 = t62 * t28;
        let t136 = t135 * t72;
        let t139 = t121 * t102;
        let tv2rhosigma0 = -0.37638687604705044352e-2 * t134 * t136 - 0.31322956504752967535e-3 * t70 * t117 * t139 + 0.18785774137723206984e-1 * t70 * t128 * t139;
        v2rhosigma[ip] += tv2rhosigma0;
        let t147 = t24 * t26 * t114;
        let t148 = t115 * t27;
        let t149 = t148 * t119;
        let t150 = 1.0 / sigma[ip];
        let t151 = t120 * t150;
        let t152 = t151 * t37;
        let t157 = t24 * t26 * t61;
        let t158 = t126 * t27;
        let t159 = t158 * t119;
        let t163 = t34 * sigma[ip];
        let t164 = 1.0 / t163;
        let t165 = t65 * t164;
        let tv2sigma20 = 0.11746108689282362825e-3 * t147 * t149 * t152 - 0.70446653016462026191e-2 * t157 * t159 * t152 + 0.90316221815976956655e-3 * t70 * t64 * t165;
        v2sigma2[ip] += tv2sigma20;
    }
}
