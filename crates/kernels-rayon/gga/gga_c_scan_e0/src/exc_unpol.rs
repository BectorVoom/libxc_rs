//! GGA_C_SCAN_E0 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_scan_e0_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.053425 * t10;
        let t13 = rmath::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 3.79785 * t13 + 0.8969 * t10 + 0.204775 * t16 + 0.123235 * t24;
        let t29 = 1.0 + 16.081979498692537 / t26;
        let t30 = rmath::ln(t29);
        let t32 = 0.0621814 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.0278125 * t10;
        let t50 = 5.1785 * t13 + 0.905775 * t10 + 0.1100325 * t16 + 0.1241775 * t24;
        let t53 = 1.0 + 29.608749977793437 / t50;
        let t54 = rmath::ln(t53);
        let t57 = 0.0197516734986138 * t43 * t45 * t54;
        let t58 = rmath::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t62 = t59 / t60;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.025 * t10;
        let t70 = 1.0 + 0.04445 * t10;
        let t71 = 1.0 / t70;
        let t72 = t68 * t71;
        let t73 = 1.0 / t59;
        let t76 = 1.0 / t66;
        let t77 = t60 * t76;
        let t79 = rmath::exp(-(-t32 + t57) * t73 * t77);
        let t80 = t79 - 1.0;
        let t81 = 1.0 / t80;
        let t82 = t73 * t81;
        let t83 = t82 * sigma[ip];
        let t84 = t72 * t83;
        let t85 = rho[ip] * rho[ip];
        let t87 = 1.0 / t7 / t85;
        let t88 = t87 * t39;
        let t89 = 1.0 / t65;
        let t91 = 1.0 / t3;
        let t93 = t18 * t91 * t5;
        let t97 = 1.0 + 0.027439371595564633 * t84 * t88 * t89 * t93;
        let t98 = pow_1_4(t97);
        let t100 = 1.0 - 1.0 / t98;
        let t103 = 1.0 + 1.0 * t100 * t80;
        let t104 = rmath::ln(t103);
        let t106 = t62 * t66 * t104;
        let tzk0 = -t32 + t57 + t106;
        zk[ip] += tzk0;
    }
}
