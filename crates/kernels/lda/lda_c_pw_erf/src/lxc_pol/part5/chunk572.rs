//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 572/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk572<F: Float>(t3975: F, t558: F, t1410: F, t640: F, t653: F, t254: F, t474: F, t252: F, t3542: F, t3638: F, t1519: F, t511: F, t198: F, t2070: F, t185: F, t1333: F, t212: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3976 = t3975 * t558;
    let t3985 = t640 * t1410;
    let t3988 = 2.0 / 9.0 * t653 * t1410;
    let t3990 = t254 * t474;
    let t3992 = 8.0 / 81.0 * t252 * t3990;
    let t3997 = 0.005877407407407408 * t3542;
    let t4013 = 0.005877407407407408 * t3638;
    let t4029 = t511 * t1519;
    let t4039 = t2070 * t198;
    let t4041 = 16.0 / 405.0 * t185 * t4039;
    let t4048 = 1.0 / t212 / t1333;
    (t3976, t3985, t3988, t3990, t3992, t3997, t4013, t4029, t4039, t4041, t4048)
}
