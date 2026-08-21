//! GGA_C_OP_XALPHA vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_xalpha_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rmath::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3(1.0 / M_PI);
        let t38 = t34 / t36;
        let t39 = M_CBRT4;
        let t40 = M_CBRT2;
        let t41 = t39 * t40;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3(t47);
        let t53 = piecewise3(t32, 0.0, t38 * t41 / t48 / 9.0);
        let t57 = t43 * t2 / 2.0 <= dens_threshold;
        let t58 = piecewise5(t44, t14, t42, t17, -t28);
        let t59 = 1.0 + t58;
        let t60 = t59 * t2;
        let t61 = pow_1_3(t60);
        let t66 = piecewise3(t57, 0.0, t38 * t41 / t61 / 9.0);
        let t67 = t53 + t66;
        let t68 = t67 == 0.0;
        let t69 = piecewise3(t68, f64::EPSILON, t67);
        let t72 = 3.90299956 / t69 + 0.5764;
        let t73 = t69 * t69;
        let t74 = t73 * t73;
        let t75 = 1.0 / t74;
        let t77 = t73 * t69;
        let t78 = 1.0 / t77;
        let t80 = 1.0 / t73;
        let t82 = 43.31320905673766 * t75 + 19.051463748196298 * t78 + 2.094820520028 * t80;
        let t83 = 1.0 / t82;
        let t84 = t72 * t83;
        let tzk0 = piecewise3(t11, 0.0, -0.25 * t21 * t84);
        zk[ip] += tzk0;
        let t87 = t2 * t2;
        let t88 = 1.0 / t87;
        let t89 = t1 * t88;
        let t90 = t3 - t89;
        let t91 = piecewise5(t13, 0.0, t16, 0.0, t90);
        let t92 = t18 * t91;
        let t94 = t2 * t72 * t83;
        let t97 = t20 * t72;
        let t99 = 0.25 * t97 * t83;
        let t100 = t38 * t39;
        let t103 = t40 / t48 / t47;
        let t104 = piecewise5(t24, 0.0, t27, 0.0, t90);
        let t105 = piecewise5(t42, 0.0, t44, 0.0, t104);
        let t107 = t105 * t2 + t45 + 1.0;
        let t111 = piecewise3(t32, 0.0, -t100 * t103 * t107 / 27.0);
        let t114 = t40 / t61 / t60;
        let t115 = piecewise5(t44, 0.0, t42, 0.0, -t104);
        let t117 = t115 * t2 + t58 + 1.0;
        let t121 = piecewise3(t57, 0.0, -t100 * t114 * t117 / 27.0);
        let t123 = piecewise3(t68, 0.0, t111 + t121);
        let t124 = t80 * t123;
        let t125 = t124 * t83;
        let t128 = t82 * t82;
        let t129 = 1.0 / t128;
        let t130 = t72 * t129;
        let t132 = 1.0 / t74 / t69;
        let t133 = t132 * t123;
        let t135 = t75 * t123;
        let t139 = -173.25283622695065 * t133 - 57.15439124458889 * t135 - 4.189641040056 * t78 * t123;
        let t140 = t130 * t139;
        let t144 = piecewise3(t11, 0.0, 0.5 * t92 * t94 - t99 + 0.97574989 * t21 * t125 + 0.25 * t21 * t140);
        let tvrho0 = t2 * t144 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t146 = -t3 - t89;
        let t147 = piecewise5(t13, 0.0, t16, 0.0, t146);
        let t148 = t18 * t147;
        let t151 = piecewise5(t24, 0.0, t27, 0.0, t146);
        let t152 = piecewise5(t42, 0.0, t44, 0.0, t151);
        let t154 = t152 * t2 + t45 + 1.0;
        let t158 = piecewise3(t32, 0.0, -t100 * t103 * t154 / 27.0);
        let t159 = piecewise5(t44, 0.0, t42, 0.0, -t151);
        let t161 = t159 * t2 + t58 + 1.0;
        let t165 = piecewise3(t57, 0.0, -t100 * t114 * t161 / 27.0);
        let t167 = piecewise3(t68, 0.0, t158 + t165);
        let t168 = t80 * t167;
        let t169 = t168 * t83;
        let t172 = t132 * t167;
        let t174 = t75 * t167;
        let t176 = t78 * t167;
        let t178 = -173.25283622695065 * t172 - 57.15439124458889 * t174 - 4.189641040056 * t176;
        let t179 = t130 * t178;
        let t183 = piecewise3(t11, 0.0, 0.5 * t148 * t94 - t99 + 0.97574989 * t21 * t169 + 0.25 * t21 * t179);
        let tvrho1 = t2 * t183 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
