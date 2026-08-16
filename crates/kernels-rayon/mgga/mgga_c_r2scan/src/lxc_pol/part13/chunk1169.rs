//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1169/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1169(t1058: f64, t1060: f64, t2207: f64, t7088: f64, t3308: f64, t37961: f64, t7368: f64, t10776: f64, t7429: f64, t10781: f64, t7505: f64, t38028: f64, t38033: f64, t39992: f64, t39996: f64, t39998: f64, t40001: f64, t40004: f64, t40007: f64) -> f64 {
    let t40011 = t2207 * t1058 * t1060 * t7088;
    let t40016 = t37961 * t3308 * t7368;
    let t40019 = t10776 * t3308 * t7429;
    let t40021 = t10781 * t7505;
    let t40023 = 0.26198215989259945075e-1_f64 * t39992 + t39996 + 0.13099107994629972538e-1_f64 * t39998 + 0.13972381860938637373e0_f64 * t40001 + 0.13099107994629972538e0_f64 * t40004 - 0.2600466522016280569e0_f64 * t40007 + 0.65495539973149862688e-2_f64 * t40011 + 0.23804984598836975486e-2_f64 * t38028 + 0.31147743054556651236e-1_f64 * t38033 - 0.2600466522016280569e0_f64 * t40016 + 0.43341108700271342816e-1_f64 * t40019 + 0.10975748638225852664e0_f64 * t40021;
    t40023
}
