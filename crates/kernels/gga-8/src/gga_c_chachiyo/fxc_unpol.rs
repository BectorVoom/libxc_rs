//! GGA_C_CHACHIYO fxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 36 shared lines across all orders.
//! Delta: 55 lines unique to fxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_chachiyo_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (36 lines) ---
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t3 = param_bp * t2;
        let t5 = pow_1_3(1.0 / M_PI);
        let t7 = M_CBRT4;
        let t8 = 1.0 / t5 * t7;
        let t9 = pow_1_3(rho[ip]);
        let t10 = t8 * t9;
        let t13 = param_cp * t1;
        let t14 = t5 * t5;
        let t16 = t7 * t7;
        let t17 = 1.0 / t14 * t16;
        let t18 = t9 * t9;
        let t19 = t17 * t18;
        let t22 = 1.0 + t3 * t10 / 3.0 + t13 * t19 / 3.0;
        let t23 = f64::ln(t22);
        let t24 = param_ap * t23;
        let t25 = param_bf * t2;
        let t28 = param_cf * t1;
        let t31 = 1.0 + t25 * t10 / 3.0 + t28 * t19 / 3.0;
        let t32 = f64::ln(t31);
        let t36 = pow_1_3(zeta_threshold);
        let t37 = t36 * t36;
        let t38 = piecewise3(1.0 <= zeta_threshold, t37, 1.0);
        let t39 = t38 * t38;
        let t42 = -2.0 * t39 * t38 + 2.0;
        let t44 = t24 + (param_af * t32 - t24) * t42;
        let t45 = M_CBRTPI;
        let t46 = t2 * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t9 / t47;
        let t53 = 1.0 + t46 * t49 * sigma[ip] / 48.0;
        let t54 = 1.0 / t44;
        let t55 = param_h * t54;
        let t56 = f64::powf(t53, t55);
        let tzk0 = t44 * t56;
        zk[ip] += tzk0;
        // --- vxc delta (29 lines) ---
        let t58 = t8 / t18;
        let t62 = t17 / t9;
        let t65 = t3 * t58 / 9.0 + 2.0 / 9.0 * t13 * t62;
        let t67 = 1.0 / t22;
        let t68 = param_ap * t65 * t67;
        let t73 = t25 * t58 / 9.0 + 2.0 / 9.0 * t28 * t62;
        let t75 = 1.0 / t31;
        let t79 = t68 + (param_af * t73 * t75 - t68) * t42;
        let t80 = rho[ip] * t79;
        let t82 = rho[ip] * t44;
        let t83 = t44 * t44;
        let t84 = 1.0 / t83;
        let t85 = param_h * t84;
        let t86 = f64::ln(t53);
        let t87 = t79 * t86;
        let t89 = t55 * t2;
        let t90 = t47 * rho[ip];
        let t92 = 1.0 / t9 / t90;
        let t93 = t45 * t92;
        let t94 = 1.0 / t53;
        let t95 = sigma[ip] * t94;
        let t96 = t93 * t95;
        let t99 = -t85 * t87 - 7.0 / 144.0 * t89 * t96;
        let t100 = t56 * t99;
        let tvrho0 = t82 * t100 + t80 * t56 + tzk0;
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t9 / rho[ip];
        let t104 = t103 * t56;
        let t106 = t46 * t94;
        let tvsigma0 = t104 * param_h * t106 / 48.0;
        vsigma[ip] += tvsigma0;
        // --- fxc delta (this level) (55 lines) ---
        let t108 = t79 * t56;
        let t110 = t44 * t56;
        let t115 = t8 / t18 / rho[ip];
        let t117 = t17 * t103;
        let t120 = -2.0 / 27.0 * t3 * t115 - 2.0 / 27.0 * t13 * t117;
        let t121 = param_ap * t120;
        let t122 = t121 * t67;
        let t123 = t65 * t65;
        let t125 = t22 * t22;
        let t126 = 1.0 / t125;
        let t127 = param_ap * t123 * t126;
        let t131 = -2.0 / 27.0 * t25 * t115 - 2.0 / 27.0 * t28 * t117;
        let t132 = param_af * t131;
        let t134 = t73 * t73;
        let t136 = t31 * t31;
        let t137 = 1.0 / t136;
        let t141 = t122 - t127 + (-param_af * t134 * t137 + t132 * t75 - t122 + t127) * t42;
        let t142 = rho[ip] * t141;
        let t146 = t99 * t99;
        let t147 = t56 * t146;
        let t150 = 1.0 / t83 / t44;
        let t151 = param_h * t150;
        let t152 = t79 * t79;
        let t153 = t152 * t86;
        let t158 = t79 * t2;
        let t159 = t85 * t158;
        let t162 = t47 * t47;
        let t164 = 1.0 / t9 / t162;
        let t166 = t45 * t164 * t95;
        let t169 = t55 * t1;
        let t170 = t45 * t45;
        let t171 = t162 * t47;
        let t173 = 1.0 / t18 / t171;
        let t175 = sigma[ip] * sigma[ip];
        let t176 = t53 * t53;
        let t177 = 1.0 / t176;
        let t178 = t175 * t177;
        let t179 = t170 * t173 * t178;
        let t182 = 2.0 * t151 * t153 - t85 * t141 * t86 + 7.0 / 72.0 * t159 * t96 + 35.0 / 216.0 * t89 * t166 - 49.0 / 6912.0 * t169 * t179;
        let t183 = t56 * t182;
        let tv2rho20 = 2.0 * t80 * t100 + 2.0 * t110 * t99 + t142 * t56 + t82 * t147 + t82 * t183 + 2.0 * t108;
        v2rho2[ip] += tv2rho20;
        let t185 = t49 * t56;
        let t192 = param_h * t2 * t45 * t94;
        let t197 = 1.0 / t18 / t162 * t56;
        let t198 = t197 * param_h;
        let t199 = t1 * t170;
        let t201 = t199 * t177 * sigma[ip];
        let tv2rhosigma0 = -t185 * param_h * t106 / 36.0 + t104 * t99 * t192 / 48.0 + 7.0 / 2304.0 * t198 * t201;
        v2rhosigma[ip] += tv2rhosigma0;
        let t205 = 1.0 / t18 / t90;
        let t206 = t205 * t56;
        let t207 = param_h * param_h;
        let t210 = t170 * t177;
        let t211 = t54 * t1 * t210;
        let t214 = t199 * t177;
        let tv2sigma20 = t206 * t207 * t211 / 768.0 - t206 * param_h * t214 / 768.0;
        v2sigma2[ip] += tv2sigma20;
    }
}
