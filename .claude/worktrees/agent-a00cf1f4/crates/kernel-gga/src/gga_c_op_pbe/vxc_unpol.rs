//! GGA_C_OP_PBE vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_op_pbe_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t4 = t1 || rho[ip] / 2.0 <= dens_threshold;
        let t5 = zeta_threshold - 1.0;
        let t6 = -t5;
        let t7 = piecewise5(t1, t5, t1, t6, 0.0);
        let t8 = t7 * t7;
        let t9 = 1.0 - t8;
        let t10 = t9 * rho[ip];
        let t11 = 1.0 + t7;
        let t14 = t11 * rho[ip] / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = t15 * t15;
        let t18 = pow_1_3(1.0 / M_PI);
        let t20 = t16 / t18;
        let t21 = M_CBRT4;
        let t22 = t20 * t21;
        let t23 = M_CBRT2;
        let t24 = t11 <= zeta_threshold;
        let t25 = 1.0 - t7;
        let t26 = t25 <= zeta_threshold;
        let t27 = piecewise5(t24, t5, t26, t6, t7);
        let t28 = 1.0 + t27;
        let t29 = t28 * rho[ip];
        let t30 = pow_1_3(t29);
        let t31 = 1.0 / t30;
        let t33 = M_CBRT6;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t36 = t35 * t35;
        let t37 = 1.0 / t36;
        let t38 = t33 * t37;
        let t39 = t23 * t23;
        let t41 = rho[ip] * rho[ip];
        let t42 = pow_1_3(rho[ip]);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t49 = 0.804e0 + 0.91464571985215458336e-2 * t38 * sigma[ip] * t39 * t45;
        let t52 = 0.1804e1 - 0.646416e0 / t49;
        let t53 = 1.0 / t52;
        let t57 = piecewise3(t14, 0.0, t22 * t23 * t31 * t53 / 9.0);
        let t61 = t25 * rho[ip] / 2.0 <= dens_threshold;
        let t62 = piecewise5(t26, t5, t24, t6, -t7);
        let t63 = 1.0 + t62;
        let t64 = t63 * rho[ip];
        let t65 = pow_1_3(t64);
        let t66 = 1.0 / t65;
        let t71 = piecewise3(t61, 0.0, t22 * t23 * t66 * t53 / 9.0);
        let t72 = t57 + t71;
        let t73 = t72 == 0.0;
        let t74 = piecewise3(t73, f64::EPSILON, t72);
        let t77 = 0.361925846e1 / t74 + 0.5764e0;
        let t78 = t74 * t74;
        let t79 = t78 * t78;
        let t80 = 1.0 / t79;
        let t82 = t78 * t74;
        let t83 = 1.0 / t82;
        let t85 = 1.0 / t78;
        let t87 = 0.320261508740743441e2 * t80 + 0.151911844324290596e2 * t83 + 0.1801312286343e1 * t85;
        let t88 = 1.0 / t87;
        let tzk0 = piecewise3(t4, 0.0, -0.25e0 * t10 * t77 * t88);
        zk[ip] += tzk0;
        let t92 = t9 * t77;
        let t96 = 1.0 / t30 / t29;
        let t102 = t21 * t31;
        let t103 = t52 * t52;
        let t104 = 1.0 / t103;
        let t106 = t20 * t102 * t104;
        let t107 = t49 * t49;
        let t108 = 1.0 / t107;
        let t109 = t108 * t33;
        let t110 = t37 * sigma[ip];
        let t111 = t41 * rho[ip];
        let t113 = 1.0 / t43 / t111;
        let t114 = t110 * t113;
        let t115 = t109 * t114;
        let t119 = piecewise3(t14, 0.0, -t22 * t23 * t96 * t53 * t28 / 27.0 + 0.35036540897419280424e-2 * t106 * t115);
        let t121 = 1.0 / t65 / t64;
        let t127 = t21 * t66;
        let t129 = t20 * t127 * t104;
        let t133 = piecewise3(t61, 0.0, -t22 * t23 * t121 * t53 * t63 / 27.0 + 0.35036540897419280424e-2 * t129 * t115);
        let t135 = piecewise3(t73, 0.0, t119 + t133);
        let t140 = t87 * t87;
        let t141 = 1.0 / t140;
        let t142 = t77 * t141;
        let t144 = 1.0 / t79 / t74;
        let t145 = t144 * t135;
        let t147 = t80 * t135;
        let t151 = -0.1281046034962973764e3 * t145 - 0.455735532972871788e2 * t147 - 0.3602624572686e1 * t83 * t135;
        let t156 = piecewise3(t4, 0.0, -0.25e0 * t92 * t88 + 0.904814615e0 * t10 * t85 * t135 * t88 + 0.25e0 * t10 * t142 * t151);
        let tvrho0 = rho[ip] * t156 + tzk0;
        vrho[ip] += tvrho0;
        let t158 = t20 * t102;
        let t159 = t104 * t108;
        let t161 = t159 * t38 * t45;
        let t164 = piecewise3(t14, 0.0, -0.13138702836532230159e-2 * t158 * t161);
        let t165 = t20 * t127;
        let t168 = piecewise3(t61, 0.0, -0.13138702836532230159e-2 * t165 * t161);
        let t170 = piecewise3(t73, 0.0, t164 + t168);
        let t175 = t144 * t170;
        let t177 = t80 * t170;
        let t179 = t83 * t170;
        let t181 = -0.1281046034962973764e3 * t175 - 0.455735532972871788e2 * t177 - 0.3602624572686e1 * t179;
        let t186 = piecewise3(t4, 0.0, 0.904814615e0 * t10 * t85 * t170 * t88 + 0.25e0 * t10 * t142 * t181);
        let tvsigma0 = rho[ip] * t186;
        vsigma[ip] += tvsigma0;
    }
}
