//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1338/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1338(t2968: f64, t8850: f64, t3061: f64, t8738: f64, t3053: f64, t8700: f64, t1085: f64, t8868: f64, t1075: f64, t1093: f64, t1095: f64, t26467: f64, t26472: f64, t26476: f64, t26479: f64, t26482: f64, t26484: f64, t26561: f64, t2917: f64, t2935: f64, t2936: f64, t2937: f64, t2974: f64, t3035: f64, t3036: f64, t3059: f64, t8754: f64, t8762: f64, t8772: f64, t8792: f64, t8795: f64, t8802: f64, t8806: f64, t8848: f64) -> f64 {
    let t26688 = t2968 * t8850;
    let t26702 = t8738 * t3061;
    let t26706 = t3053 * t8700;
    let t26715 = t8868 * t1085;
    let t26721 = 0.12414802127193579148e5_f64 * t8848 * t26688 * t2936 - 0.14035736153892489771e2_f64 * t8754 * t8802 + 0.21053604230838734656e2_f64 * t3059 * t3036 * t3053 - 0.46785787179641632568e1_f64 * t3035 * t1095 * t8738 - t26467 + 0.2077890707925103596e3_f64 * t8762 * t8792 + 0.69263023597503453196e2_f64 * t3059 * t26702 * t1093 + 0.61523382126046769581e4_f64 * t8772 * t26706 * t2917 - 24.0_f64 * t8806 * t8795 + 36.0_f64 * t2974 * t2937 * t2968 + t26472 - t26476 + t26479 + t26482 - t26484 + 0.23392893589820816284e1_f64 * t26715 * t1095 - 6.0_f64 * t2935 * t26561 * t1075;
    t26721
}
