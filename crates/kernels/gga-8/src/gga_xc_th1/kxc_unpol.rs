//! GGA_XC_TH1 kxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 52 shared lines across all orders.
//! Delta: 19 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_xc_th1_kxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    v3rho2sigma: &mut Array<f64>,
    v3rhosigma2: &mut Array<f64>,
    v3sigma3: &mut Array<f64>,
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_20: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (52 lines) ---
        let t2 = f64::powf(2.0, 1.0 / 6.0);
        let t3 = t2 * t2;
        let t4 = t3 * t3;
        let t6 = param_omega_0 * t4 * t2;
        let t7 = f64::powf(rho[ip], 1.0 / 6.0);
        let t8 = t7 * rho[ip];
        let t12 = M_CBRT2;
        let t13 = t12 * t12;
        let t14 = param_omega_1 * t13;
        let t15 = pow_1_3(rho[ip]);
        let t16 = t15 * rho[ip];
        let t20 = M_SQRT2;
        let t21 = param_omega_2 * t20;
        let t22 = f64::sqrt(rho[ip]);
        let t23 = t22 * rho[ip];
        let t27 = param_omega_3 * t12;
        let t28 = t15 * t15;
        let t29 = t28 * rho[ip];
        let t33 = param_omega_4 * t13;
        let t34 = f64::sqrt(sigma[ip]);
        let t36 = pow_1_3(zeta_threshold);
        let t38 = piecewise3(1.0 <= zeta_threshold, t36 * zeta_threshold, 1.0);
        let t43 = param_omega_5 * t20;
        let t49 = param_omega_6 * t12;
        let t55 = param_omega_7 * t2;
        let t61 = param_omega_8 * t20;
        let t62 = 1.0 / t8;
        let t64 = t38 * t38;
        let t69 = param_omega_9 * t12;
        let t70 = 1.0 / rho[ip];
        let t76 = param_omega_10 * t2;
        let t77 = t7 * t7;
        let t78 = t77 * t77;
        let t79 = t78 * t7;
        let t80 = 1.0 / t79;
        let t85 = param_omega_11;
        let t86 = 1.0 / t28;
        let t87 = t85 * t86;
        let t88 = sigma[ip] * t64;
        let t92 = param_omega_12 * t20;
        let t93 = rho[ip] * rho[ip];
        let t95 = 1.0 / t28 / t93;
        let t96 = sigma[ip] * t95;
        let t98 = t96 * t64 - t96;
        let t103 = param_omega_13 * t12;
        let t108 = param_omega_14 * t2;
        let t109 = t79 * rho[ip];
        let t113 = param_omega_15;
        let t114 = t113 * t93;
        let t117 = param_omega_20;
        let t119 = t6 * t8 / 2.0 + t14 * t16 / 2.0 + t21 * t23 / 2.0 + t27 * t29 / 2.0 + t33 * t34 * t38 / 4.0 + t43 * t7 * t34 * t38 / 4.0 + t49 * t15 * t34 * t38 / 4.0 + t55 * t22 * t34 * t38 / 4.0 + t61 * t62 * sigma[ip] * t64 / 8.0 + t69 * t70 * sigma[ip] * t64 / 8.0 + t76 * t80 * sigma[ip] * t64 / 8.0 + t87 * t88 / 8.0 + t92 * t23 * t98 / 2.0 + t103 * t29 * t98 / 2.0 + t108 * t109 * t98 / 2.0 + t114 * t98 / 2.0 + t117 * rho[ip];
        let tzk0 = t119 * t70;
        zk[ip] += tzk0;
        // --- vxc delta (15 lines) ---
        let t136 = 1.0 / t22;
        let t142 = 1.0 / t7 / t93;
        let t147 = 1.0 / t93;
        let t152 = 1.0 / t109;
        let t157 = 1.0 / t29;
        let t158 = t85 * t157;
        let t164 = t93 * rho[ip];
        let t166 = 1.0 / t28 / t164;
        let t167 = sigma[ip] * t166;
        let t170 = -8.0 / 3.0 * t167 * t64 + 8.0 / 3.0 * t167;
        let t186 = t113 * rho[ip];
        let tvrho0 = 7.0 / 12.0 * t6 * t7 + 2.0 / 3.0 * t14 * t15 + 3.0 / 4.0 * t21 * t22 + 5.0 / 6.0 * t27 * t28 + t43 * t80 * t34 * t38 / 24.0 + t49 * t86 * t34 * t38 / 12.0 + t55 * t136 * t34 * t38 / 8.0 - 7.0 / 48.0 * t61 * t142 * sigma[ip] * t64 - t69 * t147 * sigma[ip] * t64 / 8.0 - 5.0 / 48.0 * t76 * t152 * sigma[ip] * t64 - t158 * t88 / 12.0 + 3.0 / 4.0 * t92 * t22 * t98 + t92 * t23 * t170 / 2.0 + 5.0 / 6.0 * t103 * t28 * t98 + t103 * t29 * t170 / 2.0 + 11.0 / 12.0 * t108 * t79 * t98 + t108 * t109 * t170 / 2.0 + t186 * t98 + t114 * t170 / 2.0 + t117;
        vrho[ip] += tvrho0;
        let t190 = 1.0 / t34;
        let t218 = t95 * t64 - t95;
        let tvsigma0 = t33 * t190 * t38 / 8.0 + t43 * t7 * t190 * t38 / 8.0 + t49 * t15 * t190 * t38 / 8.0 + t55 * t22 * t190 * t38 / 8.0 + t61 * t62 * t64 / 8.0 + t69 * t70 * t64 / 8.0 + t76 * t80 * t64 / 8.0 + t87 * t64 / 8.0 + t92 * t23 * t218 / 2.0 + t103 * t29 * t218 / 2.0 + t108 * t109 * t218 / 2.0 + t114 * t218 / 2.0;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (18 lines) ---
        let t230 = t85 * t95;
        let t236 = t93 * t93;
        let t238 = 1.0 / t28 / t236;
        let t239 = sigma[ip] * t238;
        let t242 = 88.0 / 9.0 * t239 * t64 - 88.0 / 9.0 * t239;
        let t266 = 1.0 / t23;
        let t272 = 1.0 / t7 / t164;
        let t277 = 5.0 / 36.0 * t230 * t88 + 3.0 / 2.0 * t92 * t22 * t170 + t92 * t23 * t242 / 2.0 + 5.0 / 3.0 * t103 * t28 * t170 + t103 * t29 * t242 / 2.0 + 11.0 / 6.0 * t108 * t79 * t170 + t108 * t109 * t242 / 2.0 - 5.0 / 144.0 * t43 * t152 * t34 * t38 - t49 * t157 * t34 * t38 / 18.0 - t55 * t266 * t34 * t38 / 16.0 + 91.0 / 288.0 * t61 * t272 * sigma[ip] * t64;
        let t278 = 1.0 / t164;
        let t284 = 1.0 / t79 / t93;
        let t292 = 1.0 / t15;
        let t296 = 1.0 / t7;
        let t313 = t69 * t278 * sigma[ip] * t64 / 4.0 + 55.0 / 288.0 * t76 * t284 * sigma[ip] * t64 + 3.0 / 8.0 * t92 * t136 * t98 + 5.0 / 9.0 * t103 * t292 * t98 + 55.0 / 72.0 * t108 * t296 * t98 + 7.0 / 72.0 * t6 * t80 + 2.0 / 9.0 * t14 * t86 + 3.0 / 8.0 * t21 * t136 + 5.0 / 9.0 * t27 * t292 + t113 * t98 + 2.0 * t186 * t170 + t114 * t242 / 2.0;
        let tv2rho20 = t277 + t313;
        v2rho2[ip] += tv2rho20;
        let t342 = -8.0 / 3.0 * t166 * t64 + 8.0 / 3.0 * t166;
        let tv2rhosigma0 = t43 * t80 * t190 * t38 / 48.0 + t49 * t86 * t190 * t38 / 24.0 + t55 * t136 * t190 * t38 / 16.0 - 7.0 / 48.0 * t61 * t142 * t64 - t69 * t147 * t64 / 8.0 - 5.0 / 48.0 * t76 * t152 * t64 - t158 * t64 / 12.0 + 3.0 / 4.0 * t92 * t22 * t218 + t92 * t23 * t342 / 2.0 + 5.0 / 6.0 * t103 * t28 * t218 + t103 * t29 * t342 / 2.0 + 11.0 / 12.0 * t108 * t79 * t218 + t108 * t109 * t342 / 2.0 + t186 * t218 + t114 * t342 / 2.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t362 = 1.0 / t34 / sigma[ip];
        let tv2sigma20 = -t49 * t15 * t362 * t38 / 16.0 - t55 * t22 * t362 * t38 / 16.0 - t43 * t7 * t362 * t38 / 16.0 - t33 * t362 * t38 / 16.0;
        v2sigma2[ip] += tv2sigma20;
        // --- kxc delta (this level) (19 lines) ---
        let t386 = 1.0 / t22 / t93;
        let t392 = 1.0 / t7 / t236;
        let t397 = 1.0 / t236;
        let t403 = 1.0 / t79 / t164;
        let t414 = 1.0 / t16;
        let t419 = t236 * rho[ip];
        let t421 = 1.0 / t28 / t419;
        let t422 = sigma[ip] * t421;
        let t425 = -1232.0 / 27.0 * t422 * t64 + 1232.0 / 27.0 * t422;
        let t428 = 3.0 * t113 * t170 + 55.0 / 864.0 * t43 * t284 * t34 * t38 + 5.0 / 54.0 * t49 * t95 * t34 * t38 + 3.0 / 32.0 * t55 * t386 * t34 * t38 - 1729.0 / 1728.0 * t61 * t392 * sigma[ip] * t64 - 3.0 / 4.0 * t69 * t397 * sigma[ip] * t64 - 935.0 / 1728.0 * t76 * t403 * sigma[ip] * t64 - 35.0 / 432.0 * t6 * t152 - 4.0 / 27.0 * t14 * t157 - 3.0 / 16.0 * t21 * t266 - 5.0 / 27.0 * t27 * t414 + 3.0 * t186 * t242 + t114 * t425 / 2.0;
        let t447 = t85 * t166;
        let t468 = 9.0 / 8.0 * t92 * t136 * t170 + 5.0 / 3.0 * t103 * t292 * t170 + 55.0 / 24.0 * t108 * t296 * t170 - 3.0 / 16.0 * t92 * t266 * t98 - 5.0 / 27.0 * t103 * t414 * t98 - 55.0 / 432.0 * t108 * t62 * t98 - 10.0 / 27.0 * t447 * t88 + 9.0 / 4.0 * t92 * t22 * t242 + t92 * t23 * t425 / 2.0 + 5.0 / 2.0 * t103 * t28 * t242 + t103 * t29 * t425 / 2.0 + 11.0 / 4.0 * t108 * t79 * t242 + t108 * t109 * t425 / 2.0;
        let tv3rho30 = t428 + t468;
        v3rho3[ip] += tv3rho30;
        let t500 = 88.0 / 9.0 * t238 * t64 - 88.0 / 9.0 * t238;
        let tv3rho2sigma0 = -5.0 / 288.0 * t43 * t152 * t190 * t38 - t49 * t157 * t190 * t38 / 36.0 - t55 * t266 * t190 * t38 / 32.0 + 91.0 / 288.0 * t61 * t272 * t64 + t69 * t278 * t64 / 4.0 + 55.0 / 288.0 * t76 * t284 * t64 + 5.0 / 36.0 * t230 * t64 + 3.0 / 8.0 * t92 * t136 * t218 + 3.0 / 2.0 * t92 * t22 * t342 + t92 * t23 * t500 / 2.0 + 5.0 / 9.0 * t103 * t292 * t218 + 5.0 / 3.0 * t103 * t28 * t342 + t103 * t29 * t500 / 2.0 + 55.0 / 72.0 * t108 * t296 * t218 + 11.0 / 6.0 * t108 * t79 * t342 + t108 * t109 * t500 / 2.0 + t113 * t218 + 2.0 * t186 * t342 + t114 * t500 / 2.0;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let tv3rhosigma20 = -t43 * t80 * t362 * t38 / 96.0 - t49 * t86 * t362 * t38 / 48.0 - t55 * t136 * t362 * t38 / 32.0;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t539 = sigma[ip] * sigma[ip];
        let t541 = 1.0 / t34 / t539;
        let tv3sigma30 = 3.0 / 32.0 * t49 * t15 * t541 * t38 + 3.0 / 32.0 * t55 * t22 * t541 * t38 + 3.0 / 32.0 * t43 * t7 * t541 * t38 + 3.0 / 32.0 * t33 * t541 * t38;
        v3sigma3[ip] += tv3sigma30;
    }
}
