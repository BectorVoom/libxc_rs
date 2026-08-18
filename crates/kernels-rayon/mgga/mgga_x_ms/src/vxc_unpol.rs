//! MGGA_X_MS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ms.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_ms_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_kappa: f64,
    param_b: f64,
    param_c: f64,
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
        let t12 = 1.0 + t11;
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
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t36 = 5.0 / 972.0 * t26 * t34;
        let t37 = param_kappa + t36;
        let t41 = param_kappa * (1.0 - param_kappa / t37);
        let t42 = tau[ip] * t28;
        let t44 = 1.0 / t31 / rho[ip];
        let t47 = t42 * t44 - t34 / 8.0;
        let t48 = t47 * t47;
        let t49 = t21 * t21;
        let t52 = 1.0 / t23 / t22;
        let t55 = 1.0 - 25.0 / 81.0 * t48 * t49 * t52;
        let t56 = t55 * t55;
        let t57 = t56 * t55;
        let t58 = t48 * t47;
        let t59 = t22 * t22;
        let t60 = 1.0 / t59;
        let t63 = t48 * t48;
        let t66 = t59 * t59;
        let t67 = 1.0 / t66;
        let t70 = 1.0 + 250.0 / 243.0 * t58 * t60 + 62500.0 / 59049.0 * param_b * t63 * t48 * t67;
        let t71 = 1.0 / t70;
        let t72 = t57 * t71;
        let t73 = param_kappa + t36 + param_c;
        let t78 = param_kappa * (1.0 - param_kappa / t73) - t41;
        let t80 = t72 * t78 + t41 + 1.0;
        let t84 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t80);
        let tzk0 = 2.0 * t84;
        zk[ip] += tzk0;
        let t86 = t18 / t31;
        let t90 = param_kappa * param_kappa;
        let t91 = t37 * t37;
        let t94 = t90 / t91 * t21;
        let t95 = t25 * sigma[ip];
        let t96 = t30 * rho[ip];
        let t98 = 1.0 / t31 / t96;
        let t99 = t28 * t98;
        let t100 = t95 * t99;
        let t101 = t94 * t100;
        let t103 = t56 * t71;
        let t104 = t103 * t78;
        let t105 = t47 * t49;
        let t110 = -5.0 / 3.0 * t42 * t33 + t29 * t98 / 3.0;
        let t111 = t52 * t110;
        let t112 = t105 * t111;
        let t115 = t70 * t70;
        let t116 = 1.0 / t115;
        let t117 = t57 * t116;
        let t118 = t48 * t60;
        let t122 = param_b * t63 * t47;
        let t123 = t67 * t110;
        let t126 = 250.0 / 81.0 * t118 * t110 + 125000.0 / 19683.0 * t122 * t123;
        let t127 = t78 * t126;
        let t129 = t73 * t73;
        let t132 = t90 / t129 * t21;
        let t135 = -10.0 / 729.0 * t132 * t100 + 10.0 / 729.0 * t101;
        let t137 = -10.0 / 729.0 * t101 - 50.0 / 27.0 * t104 * t112 - t117 * t127 + t72 * t135;
        let t142 = piecewise3(t3, 0.0, -t7 * t86 * t80 / 8.0 - 3.0 / 8.0 * t7 * t20 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t84;
        vrho[ip] += tvrho0;
        let t145 = t25 * t28;
        let t146 = t145 * t33;
        let t147 = t94 * t146;
        let t149 = t78 * t47;
        let t150 = t103 * t149;
        let t151 = t49 * t52;
        let t152 = t28 * t33;
        let t153 = t151 * t152;
        let t154 = t150 * t153;
        let t156 = t118 * t152;
        let t158 = t67 * t28;
        let t160 = t122 * t158 * t33;
        let t162 = -125.0 / 324.0 * t156 - 15625.0 / 19683.0 * t160;
        let t163 = t78 * t162;
        let t167 = 5.0 / 972.0 * t132 * t146 - 5.0 / 972.0 * t147;
        let t169 = 5.0 / 972.0 * t147 + 25.0 / 108.0 * t154 - t117 * t163 + t72 * t167;
        let t173 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t169);
        let tvsigma0 = 2.0 * rho[ip] * t173;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t175 = t28 * t44;
        let t176 = t151 * t175;
        let t184 = 250.0 / 81.0 * t118 * t175 + 125000.0 / 19683.0 * t122 * t158 * t44;
        let t185 = t78 * t184;
        let t187 = -50.0 / 27.0 * t150 * t176 - t117 * t185;
        let t191 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t187);
        let tvtau0 = 2.0 * rho[ip] * t191;
        vtau[ip] += tvtau0;
    }
}
