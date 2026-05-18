//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 897/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk897<F: Float>(t16782: F, t5653: F, t4170: F, t16771: F, t4992: F, t5659: F, t86: F, t5662: F, t11913: F, t5668: F, t2038: F, t3797: F) -> (F, F, F, F, F, F) {
    let t16783 = t5653 * t16782;
    let t16784 = t4170 * t16783;
    let t16785 = t16771 * t16784;
    let t16788 = t86 * t4992 * t5659;
    let t16789 = t5662 * t16782;
    let t16790 = t4170 * t16789;
    let t16791 = t16788 * t16790;
    let t16793 = t11913 * t5668;
    let t16795 = t2038 * t3797;
    (t16783, t16785, t16789, t16791, t16793, t16795)
}
