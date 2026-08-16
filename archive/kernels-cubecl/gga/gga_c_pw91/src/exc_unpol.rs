//! GGA_C_PW91 exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pw91.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_pw91_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3::<f64>(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2::<f64>(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081824322151104822e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.62182e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3::<f64>(zeta_threshold);
        let t36 = piecewise3::<f64>(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608574643216675549e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751789702565206229e-1 * t43 * t45 * t54;
        let t58 = M_PI * M_PI;
        let t59 = pow_1_3::<f64>(t58);
        let t60 = t59 * t59;
        let t61 = t18 * t60;
        let t62 = t34 * t34;
        let t63 = piecewise3::<f64>(t33, t62, 1.0);
        let t64 = t63 * t63;
        let t65 = t64 * t63;
        let t66 = 1.0 / t59;
        let t67 = t18 * t66;
        let t68 = rho[ip] * rho[ip];
        let t70 = 1.0 / t7 / t68;
        let t72 = sigma[ip] * t70 * t39;
        let t73 = 1.0 / t64;
        let t75 = 1.0 / t3;
        let t76 = t75 * t5;
        let t77 = t73 * t18 * t76;
        let t83 = 1.0 / t60;
        let t87 = f64::exp(-0.12897460341341234505e3 * (-t32 + t57) / t65 * t1 * t83);
        let t88 = t87 - 1.0;
        let t89 = 1.0 / t88;
        let t90 = t66 * t89;
        let t91 = sigma[ip] * sigma[ip];
        let t92 = t68 * t68;
        let t94 = 1.0 / t21 / t92;
        let t95 = t91 * t94;
        let t97 = t39 * t39;
        let t98 = t64 * t64;
        let t99 = 1.0 / t98;
        let t100 = t97 * t99;
        let t101 = 1.0 / t19;
        let t102 = t101 * t6;
        let t103 = t100 * t102;
        let t106 = t72 * t77 / 96.0 + 0.27166129655589868296e-2 * t90 * t95 * t103;
        let t107 = t1 * t66;
        let t109 = t107 * t89 * sigma[ip];
        let t110 = t70 * t39;
        let t112 = t73 * t75 * t5;
        let t116 = t18 * t83;
        let t117 = t88 * t88;
        let t118 = 1.0 / t117;
        let t119 = t118 * t91;
        let t120 = t116 * t119;
        let t121 = t94 * t97;
        let t122 = t99 * t101;
        let t123 = t122 * t6;
        let t124 = t121 * t123;
        let t127 = 1.0 + 0.86931614897887578546e-1 * t109 * t110 * t112 + 0.75571056687546295931e-2 * t120 * t124;
        let t128 = 1.0 / t127;
        let t132 = 1.0 + 0.27818116767324025134e1 * t67 * t106 * t128;
        let t133 = f64::ln(t132);
        let t136 = 0.25844881434903430496e-2 * t61 * t65 * t133;
        let t137 = t2 * t59;
        let t140 = 0.2568e1 + 0.58165e1 * t10 + 0.184725e-2 * t24;
        let t143 = 1000.0 + 0.218075e4 * t10 + 118.0 * t24;
        let t144 = 1.0 / t143;
        let t146 = t140 * t144 - 0.18535714285714285714e-2;
        let t147 = t146 * t63;
        let t149 = t137 * t147 * sigma[ip];
        let t151 = pow_1_3::<f64>(9.0);
        let t152 = t151 * t151;
        let t156 = 1.0 / t21 / t68;
        let t158 = sigma[ip] * t39;
        let t162 = f64::exp(-25.0 / 18.0 * t2 * t5 * t152 * t3 * t156 * t64 * t158);
        let t163 = t76 * t162;
        let t164 = t110 * t163;
        let t166 = t149 * t164 / 2.0;
        let tzk0 = -t32 + t57 + t136 + t166;
        zk[ip] += tzk0;
    }
}
