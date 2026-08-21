//! GGA_C_FT97 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_ft97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_ft97_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = rmath::ln(2.0);
        let t2 = 1.0 - t1;
        let t3 = M_PI * M_PI;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t6 = M_CBRT3;
        let t7 = 1.0 / M_PI;
        let t8 = pow_1_3(t7);
        let t9 = t6 * t8;
        let t10 = t5 * t9;
        let t11 = M_CBRT4;
        let t12 = t11 * t11;
        let t13 = pow_1_3(rho[ip]);
        let t14 = 1.0 / t13;
        let t15 = t12 * t14;
        let t16 = M_CBRT2;
        let t17 = rmath::pow(4.0, 1.0 / 5.0);
        let t18 = t15 * t16;
        let t19 = t9 * t18;
        let t20 = rmath::pow(t19, 1.0 / 5.0);
        let t21 = t20 * t20;
        let t22 = t21 * t21;
        let t25 = rmath::exp(-0.02081897 * t17 * t22);
        let t27 = 0.942486901 + 0.349064173 * t25;
        let t28 = t27 * t27;
        let t29 = t6 * t6;
        let t30 = t8 * t8;
        let t31 = t29 * t30;
        let t32 = t31 * t11;
        let t33 = t16 * t16;
        let t34 = sigma[ip] * t33;
        let t35 = rho[ip] * rho[ip];
        let t36 = t13 * t13;
        let t38 = 1.0 / t36 / t35;
        let t42 = t8 * t7;
        let t43 = t6 * t42;
        let t44 = t43 * t12;
        let t45 = sigma[ip] * sigma[ip];
        let t46 = t45 * t16;
        let t47 = t35 * t35;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t13 / t48;
        let t52 = t44 * t46 * t50;
        let t53 = 0.0011113838714704712 * t52;
        let t54 = 1.0 + 0.04505885463888889 * t32 * t34 * t38 + t53;
        let t55 = t54 * t54;
        let t56 = t28 * t55;
        let t57 = rmath::exp(-t53);
        let t58 = t57 * t57;
        let t59 = t9 * t12;
        let t60 = sigma[ip] * t16;
        let t62 = 1.0 / t13 / t35;
        let t64 = t59 * t60 * t62;
        let t66 = 1.0 + 0.038306165027777776 * t64;
        let t67 = 1.0 / t66;
        let t68 = t58 * t67;
        let t69 = t56 * t68;
        let t70 = 1e-60 < t69;
        let t71 = piecewise3(t70, t69, 1e-60);
        let t72 = 1.0 / t71;
        let t73 = t16 * t72;
        let t75 = t10 * t15 * t73;
        let t76 = t75 / 6.0;
        let t77 = 10000000.0 <= t76;
        let t78 = xc_e1_scaled(t76);
        let t79 = t5 * t59;
        let t80 = t14 * t16;
        let t81 = rmath::sqrt(6.0);
        let t82 = t81 * t7;
        let t83 = t72 * t2;
        let t85 = t59 * t80 * t83;
        let t86 = rmath::sqrt(t85);
        let t87 = t82 * t86;
        let t90 = 3.0 + t87 / 3.0 + t75 / 3.0;
        let t92 = 3.0 + t87 + t75;
        let t93 = 1.0 / t92;
        let t94 = t72 * t90 * t93;
        let t98 = 1.0 + t79 * t80 * t94 / 3.0;
        let t100 = t90 * t93;
        let t105 = piecewise3(t77, 0.0, t5 * (-t78 * t98 + 2.0 * t100) / 4.0);
        let t106 = rmath::sqrt(t19);
        let t108 = rmath::exp(-0.544669424 * t106);
        let t110 = t17 * t17;
        let t111 = t110 * t17;
        let t114 = rmath::exp(-0.16390970575 * t111 * t21);
        let t116 = 1.247511874 - 0.859614445 * t108 + 0.812904345 * t114;
        let t117 = t116 * t116;
        let t118 = 0.1132671260325718 * t52;
        let t119 = 1.0 + t118;
        let t120 = t119 * t119;
        let t121 = t117 * t120;
        let t122 = rmath::exp(-t118);
        let t123 = t122 * t122;
        let t125 = 1.0 + 0.1000170016388889 * t64;
        let t126 = 1.0 / t125;
        let t127 = t123 * t126;
        let t128 = t121 * t127;
        let t129 = 1e-60 < t128;
        let t130 = piecewise3(t129, t128, 1e-60);
        let t131 = 1.0 / t130;
        let t132 = t16 * t131;
        let t134 = t10 * t15 * t132;
        let t135 = t134 / 6.0;
        let t136 = 10000000.0 <= t135;
        let t137 = xc_e1_scaled(t135);
        let t138 = t131 * t2;
        let t140 = t59 * t80 * t138;
        let t141 = rmath::sqrt(t140);
        let t142 = t82 * t141;
        let t145 = 3.0 + t142 / 3.0 + t134 / 3.0;
        let t147 = 3.0 + t142 + t134;
        let t148 = 1.0 / t147;
        let t149 = t131 * t145 * t148;
        let t153 = 1.0 + t79 * t80 * t149 / 3.0;
        let t155 = t145 * t148;
        let t157 = -t137 * t153 + 2.0 * t155;
        let t158 = 1.0 / t36;
        let t159 = t158 * t33;
        let t162 = 0.469508 * t106 + 0.4332925 * t19;
        let t163 = t162 * t162;
        let t164 = 1.0 / t163;
        let t168 = rmath::exp(-t32 * t159 * t164 / 4.0);
        let t172 = piecewise3(t136, 0.0, t5 * t157 * t168 / 4.0);
        let tzk0 = 2.0 * t105 + 2.0 * t172;
        zk[ip] += tzk0;
    }
}
