//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1028/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1028<F: Float>(t199: F, t3993: F, t1135: F, t566: F, t115: F, t1194: F, t4182: F, t113: F, t27: F, t4238: F, t55: F, t1183: F, t97: F) -> (F, F, F, F, F) {
    let t10492 = t3993 * t199;
    let t10494 = t1135 * t566;
    let t10500 = F::cast_from(0.1397792_f64) * t1194 * t4182 * t115;
    let t10505 = F::cast_from(0.00011806781668990758_f64) * t113 * t4238 * t27 * t55 * t115;
    let t10506 = t1183 * t97;
    (t10492, t10494, t10500, t10505, t10506)
}
