//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1263/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1263<F: Float>(t2753: F, t754: F, t936: F, t97: F, t1786: F, t1789: F, t409: F, t10976: F, t10993: F, t14758: F, t14761: F, t14765: F, t8032: F, t8034: F, t8039: F, t8043: F, t8047: F) -> (F, F) {
    let t19055 = t2753 * t754 * t97 * t936;
    let t19063 = t409 * t2753 * t1786 * t1789;
    let t19069 = 2.0 * t10993 - 0.4564036537785185 * t14758 + 0.6327242966164848 * t19063 + t10976 + 0.9480012043054112 * t14761 + t8047 - t8039 + 0.8215265768013333 * t14765 - 2.530897186465939 * t8032 - 0.4564036537785185 * t8034 + t8043;
    (t19055, t19069)
}
