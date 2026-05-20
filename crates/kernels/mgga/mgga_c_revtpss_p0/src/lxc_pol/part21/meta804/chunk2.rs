//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2925/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2925<F: Float>(t3056: F, t4742: F, t378: F, t11200: F, t379: F, t1678: F, t3043: F, t3075: F, t4772: F, t3259: F, t4746: F, t1000: F, t1079: F, t1097: F, t11214: F, t11224: F, t11804: F, t12043: F, t12177: F, t15579: F, t16239: F, t16284: F, t16305: F, t16313: F, t16318: F, t16328: F, t16344: F, t16352: F, t16374: F, t16592: F, t3047: F, t3058: F, t3060: F, t3063: F, t3067: F, t3076: F, t3264: F, t3269: F, t3270: F, t3325: F, t33754: F, t4941: F, t989: F, t995: F, t996: F) -> (F, F, F) {
    let t53166 = t4742 * t3056;
    let t53167 = t53166 * t378;
    let t53174 = t11200 * t379;
    let t53180 = t3043 * t1678;
    let t53192 = t4772 * t3075;
    let t53208 = t4746 * t3259;
    let t53217 = F::cast_from(0.39512695097613069591e1_f64) * t53167 * t3060 - F::cast_from(0.19756347548806534796e1_f64) * t16374 * t3076 + F::cast_from(0.19756347548806534796e1_f64) * t11214 * t4941 + F::cast_from(0.11853808529283920877e2_f64) * t53174 * t16313 * t12177 + F::cast_from(0.19756347548806534796e1_f64) * t989 * t16239 - F::cast_from(0.19756347548806534796e1_f64) * t53180 * t1097 - F::cast_from(0.11853808529283920877e2_f64) * t53174 * t33754 * t11804 + F::cast_from(0.19756347548806534796e1_f64) * t3063 * t15579 - F::cast_from(0.39512695097613069591e1_f64) * t995 * t3269 * t4772 * t3270 + F::cast_from(0.39512695097613069591e1_f64) * t3058 * t996 * t53192 - F::cast_from(0.19756347548806534796e1_f64) * t3264 * t16592 + F::cast_from(0.39512695097613069591e1_f64) * t16284 * t12043 + F::cast_from(0.39512695097613069591e1_f64) * t16305 * t3067 + F::cast_from(0.39512695097613069591e1_f64) * t3063 * t16328 + F::cast_from(0.39512695097613069591e1_f64) * t3264 * t16318 - F::cast_from(0.39512695097613069591e1_f64) * t11224 * t16344 - F::cast_from(0.19756347548806534796e1_f64) * t53208 * t1000 + F::cast_from(0.19756347548806534796e1_f64) * t995 * t1079 * t4772 * t3325 + F::cast_from(0.19756347548806534796e1_f64) * t3047 * t16352;
    (t53166, t53192, t53217)
}
