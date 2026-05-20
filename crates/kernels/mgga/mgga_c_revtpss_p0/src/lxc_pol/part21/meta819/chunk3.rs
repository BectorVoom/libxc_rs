//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3024/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3024<F: Float>(t3057: F, t4930: F, t15886: F, t378: F, t3046: F, t1000: F, t1079: F, t1097: F, t11123: F, t11178: F, t11190: F, t11224: F, t12040: F, t12174: F, t16255: F, t16275: F, t16287: F, t16292: F, t16302: F, t16340: F, t16371: F, t1652: F, t16603: F, t3052: F, t3058: F, t3059: F, t3060: F, t3063: F, t3066: F, t3076: F, t3268: F, t3271: F, t3326: F, t43637: F, t43670: F, t4747: F, t4752: F, t4773: F, t4935: F, t5015: F, t54955: F, t996: F) -> F {
    let t55413 = t3057 * t4930;
    let t55416 = t15886 * t378;
    let t55421 = t3046 * t4930;
    let t55453 = -F::cast_from(0.79025390195226139182e1_f64) * t16603 * t3268 * t5015 * t3066 - F::cast_from(0.11853808529283920877e2_f64) * t43637 * t16275 + F::cast_from(0.39512695097613069591e1_f64) * t55413 * t3060 - F::cast_from(0.19756347548806534796e1_f64) * t55416 * t1097 + F::cast_from(0.79025390195226139182e1_f64) * t11224 * t16292 - F::cast_from(0.39512695097613069591e1_f64) * t55421 * t1000 - F::cast_from(0.19756347548806534796e1_f64) * t3063 * t16287 - F::cast_from(0.19756347548806534796e1_f64) * t16302 * t3076 - F::cast_from(0.39512695097613069591e1_f64) * t3058 * t1079 * t5015 * t3059 - F::cast_from(0.65854491829355115987e0_f64) * t4935 * t12174 - F::cast_from(0.39512695097613069591e1_f64) * t4747 * t12040 + F::cast_from(0.79025390195226139182e1_f64) * t3052 * t16255 + F::cast_from(0.39512695097613069591e1_f64) * t4935 * t11178 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t996 * t54955 - F::cast_from(0.39512695097613069591e1_f64) * t4752 * t11123 - F::cast_from(0.65854491829355115987e0_f64) * t43670 * t1652 - F::cast_from(0.19756347548806534796e1_f64) * t11190 * t4773 + F::cast_from(0.39512695097613069591e1_f64) * t16371 * t3271 - F::cast_from(0.19756347548806534796e1_f64) * t16340 * t3326;
    t55453
}
