//! MGGA_X_MBEEF exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeef.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mbeef_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t39 = 0.65124e1 + t26 * t36 / 24.0;
        let t40 = 1.0 / t39;
        let t41 = t34 * t40;
        let t42 = t27 * t41;
        let t44 = tau[ip] * t29;
        let t46 = 1.0 / t31 / rho[ip];
        let t52 = 5.0 / 9.0 * (t44 * t46 - t36 / 8.0) * t21 * t25;
        let t53 = 10000.0 <= t52;
        let t54 = 10000.0 < t52;
        let t55 = piecewise3(t54, t52, 10000.0);
        let t56 = t55 * t55;
        let t59 = t56 * t55;
        let t60 = 1.0 / t59;
        let t61 = t56 * t56;
        let t62 = 1.0 / t61;
        let t65 = piecewise3(t54, 10000.0, t52);
        let t66 = t65 * t65;
        let t67 = 1.0 - t66;
        let t68 = t67 * t67;
        let t69 = t68 * t67;
        let t70 = t66 * t65;
        let t71 = 1.0 + t70;
        let t73 = t70 * t71 + 1.0;
        let t74 = 1.0 / t73;
        let t76 = piecewise3(t53, 1.0 - 3.0 / t56 - t60 + 3.0 * t62, -t69 * t74);
        let t77 = t76 * t76;
        let t78 = t77 * t76;
        let t79 = t77 * t77;
        let t80 = t79 * t78;
        let t83 = t42 / 12.0 - 1.0;
        let t84 = t83 * t83;
        let t85 = t84 * t83;
        let t87 = t84 * t84;
        let t88 = t87 * t84;
        let t91 = t87 * t85;
        let t93 = t87 * t83;
        let t97 = 429.0 / 16.0 * t91 - 693.0 / 16.0 * t93 + 315.0 / 16.0 * t85 - 35.0 / 192.0 * t42 + 35.0 / 16.0;
        let t99 = t79 * t76;
        let t103 = 429.0 / 16.0 * t80 - 693.0 / 16.0 * t99 + 315.0 / 16.0 * t78 - 35.0 / 16.0 * t76;
        let t106 = t79 * t77;
        let t110 = -5.0 / 16.0 + 231.0 / 16.0 * t106 - 315.0 / 16.0 * t79 + 105.0 / 16.0 * t77;
        let t116 = 63.0 / 8.0 * t99 - 35.0 / 4.0 * t78 + 15.0 / 8.0 * t76;
        let t121 = 3.0 / 8.0 + 35.0 / 8.0 * t79 - 15.0 / 4.0 * t77;
        let t126 = 5.0 / 2.0 * t78 - 3.0 / 2.0 * t76;
        let t130 = -1.0 / 2.0 + 3.0 / 2.0 * t77;
        let t133 = t97 * t76;
        let t139 = 63.0 / 8.0 * t93 - 35.0 / 4.0 * t85 + 5.0 / 32.0 * t42 - 15.0 / 8.0;
        let t146 = -0.13022208355989583333e-1 * t42 + 0.19735677658125e-4 * t80 + 0.497944638409375e0 * t85 + 0.80024660533125e-1 * t88 - 0.4373652639371875e-2 * t76 + 0.888525527e-8 * t97 * t103 - 0.774224962e-8 * t97 * t110 - 0.338128188e-7 * t97 * t116 + 0.554588743e-7 * t97 * t121 + 0.505920757e-7 * t97 * t126 - 0.27652468e-6 * t97 * t130 + 0.940675747e-2 * t133 - 0.138056183978125e0 * t87 - 0.138472194e-7 * t139 * t110 - 0.376702959e-7 * t139 * t116 + 0.162238741e-6 * t139 * t121;
        let t151 = t139 * t76;
        let t155 = 3.0 / 8.0 + 35.0 / 8.0 * t87 - 15.0 / 4.0 * t84;
        let t168 = t155 * t76;
        let t172 = 5.0 / 2.0 * t85 - t42 / 8.0 + 3.0 / 2.0;
        let t185 = -0.896771404e-2 * t139 * t126 - 0.188495102e-1 * t139 * t130 - 0.884148272e-2 * t151 - 0.493824365e-8 * t155 * t103 + 0.912223751e-8 * t155 * t110 + 0.209603871e-7 * t155 * t116 - 0.790811707e-7 * t155 * t121 + 0.631891628e-2 * t155 * t126 - 0.182911291e-1 * t155 * t130 + 0.162638575e-1 * t168 + 0.674910119e-8 * t172 * t103 - 0.216860568e-7 * t172 * t110 + 0.896739466e-3 * t172 * t116 + 0.339308972e-2 * t172 * t121 - 0.845508103e-2 * t172 * t126 + 0.280678872e-1 * t172 * t130;
        let t187 = t172 * t76;
        let t190 = -1.0 / 2.0 + 3.0 / 2.0 * t84;
        let t206 = t190 * t76;
        let t218 = -0.182177954e-1 * t187 - 0.223014657e-8 * t190 * t103 - 0.395061199588125e0 * t93 - 0.945883103563125e-3 * t99 + 0.4646102821846875e-2 * t78 + 0.668980219e-8 * t190 * t110 - 0.35104103e-3 * t190 * t116 + 0.182906057e-2 * t190 * t121 + 0.293253041e-2 * t190 * t126 - 0.150103636e-1 * t190 * t130 - 0.43464346e-1 * t206 - 0.940351563e-5 * t83 * t103 - 0.514204676e-4 * t83 * t110 + 0.822139896e-3 * t83 * t116 + 0.119130546e-2 * t83 * t121 - 0.303347141e-2 * t83 * t126;
        let t221 = t83 * t76;
        let t226 = -5.0 / 16.0 + 231.0 / 16.0 * t88 - 315.0 / 16.0 * t87 + 105.0 / 16.0 * t84;
        let t239 = t226 * t76;
        let t248 = 0.1380567225218996875e1 - 0.879090772e-2 * t83 * t130 + 0.100339208e0 * t221 - 0.691592964e-8 * t226 * t103 + 0.694482484e-8 * t226 * t110 + 0.236391411e-7 * t226 * t116 - 0.416393106e-7 * t226 * t121 - 0.265114646e-7 * t226 * t126 + 0.169805915e-6 * t226 * t130 - 0.957417512e-2 * t239 + 0.850272392e-8 * t139 * t103 + 0.106025815520625e0 * t91 - 0.80008813355625e-4 * t106 + 0.3020715669803125e-2 * t79 + 0.7031826877565625e-2 * t77 - 0.92294814328125e-1 * t84;
        let t250 = t146 + t185 + t218 + t248;
        let t254 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t250);
        let tzk0 = 2.0 * t254;
        zk[ip] += tzk0;
    }
}
