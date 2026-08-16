//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 607/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk607<F: Float>(t1635: F, t645: F, t4044: F, t2060: F, t5898: F, t903: F, t1614: F, t649: F, t27: F, t2139: F, t2333: F, t7508: F) -> (F, F, F, F, F, F, F) {
    let t8548 = t645 * t1635;
    let t8549 = t4044 * t8548;
    let t8551 = t2060 * t5898;
    let t8552 = t903 * t8551;
    let t8561 = t649 * t1614;
    let t8562 = t27 * t8561;
    let t8563 = t2139 * t8562;
    let t8565 = t7508 * t2333;
    (t8548, t8549, t8551, t8552, t8562, t8563, t8565)
}
