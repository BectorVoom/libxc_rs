//! MGGA_X_MBEEFVDW exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeefvdw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mbeefvdw_exc_unpol(
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
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = t11 + 1.0;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = t26 * sigma[ip];
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = sigma[ip] * t29;
        let t36 = t35 * t33;
        let t39 = 6.5124 + t26 * t36 / 24.0;
        let t40 = 1.0 / t39;
        let t41 = t34 * t40;
        let t42 = t27 * t41;
        let t44 = t42 / 12.0 - 1.0;
        let t45 = tau[ip] * t29;
        let t47 = 1.0 / t31 / rho[ip];
        let t53 = 5.0 / 9.0 * (t45 * t47 - t36 / 8.0) * t21 * t25;
        let t54 = 10000.0 <= t53;
        let t55 = 10000.0 < t53;
        let t56 = piecewise3(t55, t53, 10000.0);
        let t57 = t56 * t56;
        let t60 = t57 * t56;
        let t61 = 1.0 / t60;
        let t62 = t57 * t57;
        let t63 = 1.0 / t62;
        let t66 = piecewise3(t55, 10000.0, t53);
        let t67 = t66 * t66;
        let t68 = 1.0 - t67;
        let t69 = t68 * t68;
        let t70 = t69 * t68;
        let t71 = t67 * t66;
        let t72 = 1.0 + t71;
        let t74 = t71 * t72 + 1.0;
        let t75 = 1.0 / t74;
        let t77 = piecewise3(t54, 1.0 - 3.0 / t57 - t61 + 3.0 * t63, -t70 * t75);
        let t78 = t77 * t77;
        let t79 = t78 * t78;
        let t82 = 3.0 / 8.0 + 35.0 / 8.0 * t79 - 15.0 / 4.0 * t78;
        let t85 = t78 * t77;
        let t88 = 5.0 / 2.0 * t85 - 3.0 / 2.0 * t77;
        let t92 = -1.0 / 2.0 + 3.0 / 2.0 * t78;
        let t95 = t44 * t77;
        let t99 = t44 * t44;
        let t100 = t99 * t99;
        let t106 = 3.0 / 8.0 + 35.0 / 8.0 * t100 - 15.0 / 4.0 * t99;
        let t113 = -1.00478906e-07 * t44 * t82 - 0.00608338264 * t44 * t88 + 0.0318024096 * t44 * t92 + 0.0453837246 * t95 - 0.06972770593 * t77 + 0.0217681859775 * t78 + 0.00618699843125 * t100 + 0.01214700985 * t42 - 0.0851282539125 * t99 - 3.40722258e-09 * t106 * t82 + 5.74317889e-08 * t106 * t88 - 5.00749348e-07 * t106 * t92;
        let t114 = t106 * t77;
        let t116 = t99 * t44;
        let t119 = 5.0 / 2.0 * t116 - t42 / 8.0 + 3.0 / 2.0;
        let t126 = t119 * t77;
        let t129 = -1.0 / 2.0 + 3.0 / 2.0 * t99;
        let t136 = t129 * t77;
        let t141 = 1.0451438955835 + 9.19317034e-07 * t114 + 3.97324768e-09 * t119 * t82 - 5.49909413e-08 * t119 * t88 + 1.33707403e-07 * t119 * t92 + 0.0192374554 * t126 + 2.01895739e-07 * t129 * t82 - 6.57949254e-07 * t129 * t88 - 0.00521818079 * t129 * t92 - 0.0222650139 * t136 + 0.00061919587625 * t79 - 0.050282912 * t116 + 0.00351985355 * t85;
        let t142 = t113 + t141;
        let t146 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t142);
        let tzk0 = 2.0 * t146;
        zk[ip] += tzk0;
    }
}
