//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 785/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk785<F: Float>(t3218: F, t4813: F, t1021: F, t1092: F, t1121: F, t1767: F) -> (F, F, F, F) {
    let t4814 = t3218 * t4813;
    let t4815 = t1021 * t4814;
    let t4816 = t1092 * t4815;
    let t4818 = t1767 * t1121;
    (t4814, t4815, t4816, t4818)
}
