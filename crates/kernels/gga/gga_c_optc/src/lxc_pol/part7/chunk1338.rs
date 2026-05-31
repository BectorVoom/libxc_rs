//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1338/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1338<F: Float>(t2968: F, t8850: F, t3061: F, t8738: F, t3053: F, t8700: F, t1085: F, t8868: F, t1075: F, t1093: F, t1095: F, t26467: F, t26472: F, t26476: F, t26479: F, t26482: F, t26484: F, t26561: F, t2917: F, t2935: F, t2936: F, t2937: F, t2974: F, t3035: F, t3036: F, t3059: F, t8754: F, t8762: F, t8772: F, t8792: F, t8795: F, t8802: F, t8806: F, t8848: F) -> F {
    let t26688 = t2968 * t8850;
    let t26702 = t8738 * t3061;
    let t26706 = t3053 * t8700;
    let t26715 = t8868 * t1085;
    let t26721 = F::cast_from(0.12414802127193579148e5_f64) * t8848 * t26688 * t2936 - F::cast_from(0.14035736153892489771e2_f64) * t8754 * t8802 + F::cast_from(0.21053604230838734656e2_f64) * t3059 * t3036 * t3053 - F::cast_from(0.46785787179641632568e1_f64) * t3035 * t1095 * t8738 - t26467 + F::cast_from(0.2077890707925103596e3_f64) * t8762 * t8792 + F::cast_from(0.69263023597503453196e2_f64) * t3059 * t26702 * t1093 + F::cast_from(0.61523382126046769581e4_f64) * t8772 * t26706 * t2917 - F::cast_from(24.0_f64) * t8806 * t8795 + F::cast_from(36.0_f64) * t2974 * t2937 * t2968 + t26472 - t26476 + t26479 + t26482 - t26484 + F::cast_from(0.23392893589820816284e1_f64) * t26715 * t1095 - F::cast_from(6.0_f64) * t2935 * t26561 * t1075;
    t26721
}
