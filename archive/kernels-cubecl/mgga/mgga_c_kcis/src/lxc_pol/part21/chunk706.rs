//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 706/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk706<F: Float>(t7718: F, t7719: F, t1020: F, t1086: F, t2179: F, t303: F, t1094: F, t342: F) -> (F, F, F, F, F) {
    let t7720 = t7718 * t7719;
    let t7721 = t1020 * t7720;
    let t7723 = t1086 * t2179;
    let t7724 = t303 * t7723;
    let t7726 = t342 * t1094;
    (t7720, t7721, t7723, t7724, t7726)
}
