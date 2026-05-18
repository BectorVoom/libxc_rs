//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1084/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1084<F: Float>(t3316: F, t3353: F, t4114: F, t6497: F, t2183: F, t4140: F, t4117: F, t6585: F, t791: F, t3324: F, t3329: F, t2194: F, t4121: F) -> (F, F, F, F, F, F, F) {
    let t10561 = F::new(2.0) * t3316 * t3353;
    let t10563 = F::new(2.0) * t6497 * t4114;
    let t10565 = F::new(1.0) * t2183 * t4140;
    let t10566 = t6585 * t4117;
    let t10567 = t10566 * t791;
    let t10569 = t3324 * t3329;
    let t10571 = t2194 * t4121;
    (t10561, t10563, t10565, t10566, t10567, t10569, t10571)
}
