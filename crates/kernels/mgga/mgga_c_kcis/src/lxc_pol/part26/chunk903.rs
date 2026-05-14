//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 903/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk903<F: Float>(t1529: F, t7314: F, t2050: F, t4291: F, t5905: F, t17396: F, t492: F, t6029: F, t21038: F, t5904: F, t5903: F, t1517: F, t167: F, t5987: F, t531: F, t7190: F) -> (F, F, F, F, F, F, F) {
    let t22461 = t1529 * t7314;
    let t22463 = t2050 * t4291;
    let t22464 = t22463 * t5905;
    let t22466 = t17396 * t492;
    let t22467 = t22466 * t6029;
    let t22470 = t5904 * t21038;
    let t22471 = t5903 * t22470;
    let t22498 = t1517 * t5987 * t167;
    let t22503 = t7190 * t531;
    (t22461, t22464, t22467, t22470, t22471, t22498, t22503)
}
