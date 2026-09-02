//! MGGA_C_RPPSCAN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rppscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rppscan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t11 = t5 * t7 / t8;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = rmath::sqrt(t11);
        let t17 = pow_3_2(t11);
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t27 = 3.79785 * t14 + 0.8969 * t11 + 0.204775 * t17 + 0.123235 * t25;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = rmath::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(t34, t35 * zeta_threshold, 1.0);
        let t39 = 2.0 * t37 - 2.0;
        let t40 = M_CBRT2;
        let t41 = t40 - 1.0;
        let t43 = 1.0 / t41 / 2.0;
        let t44 = t39 * t43;
        let t46 = 1.0 + 0.0278125 * t11;
        let t51 = 5.1785 * t14 + 0.905775 * t11 + 0.1100325 * t17 + 0.1241775 * t25;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = rmath::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let t59 = rmath::ln(2.0);
        let t60 = 1.0 - t59;
        let t61 = M_PI * M_PI;
        let t63 = t60 / t61;
        let t64 = t35 * t35;
        let t65 = piecewise3(t34, t64, 1.0);
        let t66 = t65 * t65;
        let t67 = t66 * t65;
        let t69 = 1.0 + 0.025 * t11;
        let t71 = 1.0 + 0.04445 * t11;
        let t72 = 1.0 / t71;
        let t73 = t69 * t72;
        let t74 = 1.0 / t60;
        let t77 = 1.0 / t67;
        let t78 = t61 * t77;
        let t80 = rmath::exp(-(-t33 + t58) * t74 * t78);
        let t81 = t80 - 1.0;
        let t82 = 1.0 / t81;
        let t83 = t74 * t82;
        let t84 = t83 * sigma[ip];
        let t85 = t73 * t84;
        let t86 = rho[ip] * rho[ip];
        let t88 = 1.0 / t8 / t86;
        let t89 = t88 * t40;
        let t90 = 1.0 / t66;
        let t92 = 1.0 / t4;
        let t94 = t19 * t92 * t6;
        let t98 = 1.0 + 0.027439371595564633 * t85 * t89 * t90 * t94;
        let t99 = pow_1_4(t98);
        let t101 = 1.0 - 1.0 / t99;
        let t104 = 1.0 + 1.0 * t101 * t81;
        let t105 = rmath::ln(t104);
        let t107 = t63 * t67 * t105;
        let t109 = 1.0 / t22 / rho[ip];
        let t112 = 1.0 / t22 / t86;
        let t115 = tau[ip] * t109 - sigma[ip] * t112 / 8.0;
        let t116 = M_CBRT6;
        let t117 = t116 * t116;
        let t118 = pow_1_3(t61);
        let t119 = t118 * t118;
        let t123 = param_eta * sigma[ip];
        let t126 = 3.0 / 20.0 * t117 * t119 * t40 + t123 * t112 / 8.0;
        let t127 = 1.0 / t126;
        let t128 = t115 * t127;
        let t129 = t128 <= 2.5;
        let t130 = 2.5 < t128;
        let t131 = piecewise3(t130, 2.5, t128);
        let t133 = t131 * t131;
        let t135 = t133 * t131;
        let t137 = t133 * t133;
        let t139 = t137 * t131;
        let t141 = t137 * t133;
        let t146 = piecewise3(t130, t128, 2.5);
        let t147 = 1.0 - t146;
        let t150 = rmath::exp(1.5 / t147);
        let t152 = piecewise3(t129, 1.0 - 0.64 * t131 - 0.4352 * t133 - 1.535685604549 * t135 + 3.061560252175 * t137 - 1.915710236206 * t139 + 0.516884468372 * t141 - 0.051848879792 * t137 * t135, -0.7 * t150);
        let t155 = 1.0 + 0.04445 * t14 + 0.03138525 * t11;
        let t156 = 1.0 / t155;
        let t159 = rmath::exp(1.0 * t156);
        let t160 = t159 - 1.0;
        let t161 = 1.0 / t119;
        let t162 = t116 * t161;
        let t163 = t40 * t40;
        let t164 = t163 * sigma[ip];
        let t168 = 1.0 + 0.02133764210437636 * t162 * t164 * t112;
        let t169 = pow_1_4(t168);
        let t171 = 1.0 - 1.0 / t169;
        let t173 = t160 * t171 + 1.0;
        let t174 = rmath::ln(t173);
        let t180 = 1.0 - 2.363 * t41 * t39 * t43;
        let t182 = (-0.0285764 * t156 + 0.0285764 * t174) * t180 + t33 - t58 - t107;
        let t183 = t152 * t182;
        let tzk0 = -t33 + t58 + t107 + t183;
        zk[ip] += tzk0;
    }
}
