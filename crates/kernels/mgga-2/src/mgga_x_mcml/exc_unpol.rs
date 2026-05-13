//! MGGA_X_MCML exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mcml.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mcml_exc_unpol(
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
        let t44 = t42 / 12.0 - 1.0;
        let t45 = t44 * t44;
        let t46 = t45 * t45;
        let t47 = t46 * t45;
        let t50 = t45 * t44;
        let t52 = tau[ip] * t29;
        let t54 = 1.0 / t31 / rho[ip];
        let t60 = 5.0 / 9.0 * (t52 * t54 - t36 / 8.0) * t21 * t25;
        let t61 = 10000.0 <= t60;
        let t62 = 10000.0 < t60;
        let t63 = piecewise3(t62, t60, 10000.0);
        let t64 = t63 * t63;
        let t67 = t64 * t63;
        let t68 = 1.0 / t67;
        let t70 = t64 * t64;
        let t71 = 1.0 / t70;
        let t74 = piecewise3(t62, 10000.0, t60);
        let t75 = t74 * t74;
        let t76 = 1.0 - t75;
        let t77 = t76 * t76;
        let t78 = t77 * t76;
        let t79 = t75 * t74;
        let t81 = 1.0 + 4.0 * t79;
        let t83 = t79 * t81 + 1.0;
        let t84 = 1.0 / t83;
        let t86 = piecewise3(t61, 3.0 / 4.0 / t64 + t68 / 16.0 - 3.0 / 4.0 * t71 - 1.0 / 4.0, t78 * t84);
        let t88 = t86 * t86;
        let t89 = t88 * t88;
        let t90 = t89 * t86;
        let t92 = t46 * t50;
        let t94 = t46 * t44;
        let t96 = t88 * t86;
        let t98 = t89 * t88;
        let t103 = t89 * t96;
        let t109 = 429.0 / 16.0 * t92 - 693.0 / 16.0 * t94 + 315.0 / 16.0 * t50 - 35.0 / 192.0 * t42 + 35.0 / 16.0;
        let t112 = 3.0 / 8.0 + 35.0 / 8.0 * t89 - 15.0 / 4.0 * t88;
        let t117 = 5.0 / 2.0 * t96 - 3.0 / 2.0 * t86;
        let t121 = -1.0 / 2.0 + 3.0 / 2.0 * t88;
        let t124 = 0.22419222998949863625e-1 * t47 - 0.1047053293912749375e-2 * t46 + 0.7416880187036191562e-2 * t50 + 0.20748619661467272631e0 * t86 + 0.8753451580964013919e-1 * t90 + 0.15682422300093093188e-1 * t92 - 0.15887583418757175563e-1 * t94 - 0.32121495135261672812e-1 * t96 - 0.28551704175417885e-1 * t98 + 0.294397262786656575e-1 * t89 - 0.58828844909941371e-2 * t88 - 0.37102687351218925312e0 * t45 - 0.67464548655177289688e-1 * t103 + 0.245752591853626e-2 * t109 * t112 + 0.1243327883803539e-1 * t109 * t117 + 0.1421391023843761e-2 * t109 * t121;
        let t125 = t109 * t86;
        let t130 = -5.0 / 16.0 + 231.0 / 16.0 * t47 - 315.0 / 16.0 * t46 + 105.0 / 16.0 * t45;
        let t135 = 429.0 / 16.0 * t103 - 693.0 / 16.0 * t90 + 315.0 / 16.0 * t96 - 35.0 / 16.0 * t86;
        let t141 = -5.0 / 16.0 + 231.0 / 16.0 * t98 - 315.0 / 16.0 * t89 + 105.0 / 16.0 * t88;
        let t147 = 63.0 / 8.0 * t90 - 35.0 / 4.0 * t96 + 15.0 / 8.0 * t86;
        let t154 = 5.0 / 2.0 * t50 - t42 / 8.0 + 3.0 / 2.0;
        let t163 = t154 * t86;
        let t166 = -1.0 / 2.0 + 3.0 / 2.0 * t45;
        let t179 = 0.3837976998664341e-3 * t125 + 0.3807158595350892e-3 * t130 * t135 + 0.4260858412001439e-3 * t130 * t141 + 0.1136485825094485e-2 * t130 * t147 + 0.4230264400260503e-3 * t130 * t112 + 0.1672905908063297e-3 * t154 * t147 - 0.2494950550547465e-2 * t154 * t112 + 0.3712786171321043e-2 * t154 * t117 - 0.7090296813211244e-3 * t154 * t121 - 0.1030571429426108e-1 * t163 - 0.1175614476758423e-2 * t166 * t135 - 0.1288306127279617e-2 * t166 * t141 - 0.1189668304951413e-2 * t166 * t147 - 0.1863882881010248e-2 * t166 * t112 - 0.9641371299507833e-3 * t166 * t117 - 0.1153807045825489e-2 * t166 * t121;
        let t181 = t166 * t86;
        let t195 = t44 * t86;
        let t207 = 63.0 / 8.0 * t94 - 35.0 / 4.0 * t50 + 5.0 / 32.0 * t42 - 15.0 / 8.0;
        let t210 = t207 * t86;
        let t214 = 3.0 / 8.0 + 35.0 / 8.0 * t46 - 15.0 / 4.0 * t45;
        let t219 = -0.1437960658302686e-1 * t181 + 0.1940164714223896e-2 * t44 * t135 + 0.1491587478361034e-2 * t44 * t141 + 0.2007295399058147e-2 * t44 * t147 + 0.2915285520983635e-2 * t44 * t112 + 0.2125332357775206e-2 * t44 * t117 + 0.179463855686441e-2 * t44 * t121 + 0.1179363564823021e0 * t195 - 0.3695503801501715e-3 * t109 * t135 - 0.3682519432462936e-3 * t109 * t141 + 0.1522474179598972e-2 * t109 * t147 - 0.13465921726261020182e-1 * t42 + 0.6670848599065867e-2 * t207 * t121 - 0.257733338272708e-3 * t210 + 0.3212943141118693e-5 * t214 * t135 + 0.2776060240069905e-3 * t214 * t141;
        let t228 = t214 * t86;
        let t238 = t130 * t86;
        let t250 = 0.13502664484515602222e1 - 0.2721968500889238e-3 * t214 * t147 + 0.4187827907710905e-3 * t214 * t112 + 0.1282471852770764e-2 * t214 * t117 + 0.137028863545747e-3 * t214 * t121 + 0.1683215086686233e-1 * t228 + 0.4312411759243052e-3 * t154 * t135 - 0.6058496834176058e-3 * t154 * t141 - 0.6510071882485726e-2 * t130 * t117 - 0.5498112922165805e-2 * t130 * t121 + 0.2334616776649133e-2 * t238 - 0.2202759704065197e-3 * t207 * t135 - 0.1622621390953226e-2 * t207 * t141 - 0.5869916483960576e-3 * t207 * t147 - 0.1009981263546227e-2 * t207 * t112 + 0.2262886186270548e-3 * t207 * t117;
        let t252 = t124 + t179 + t219 + t250;
        let t256 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t252);
        let tzk0 = 2.0 * t256;
        zk[ip] += tzk0;
    }
}
