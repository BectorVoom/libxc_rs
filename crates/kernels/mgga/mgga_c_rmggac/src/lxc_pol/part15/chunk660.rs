//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 660/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk660<F: Float>(t29: F, t3899: F, t117: F, t551: F, t234: F, t3350: F, t7254: F) -> (F, F, F, F) {
    let t14366 = t3899 * t29;
    let t15093 = t551 * t117;
    let t15280 = t234 * t551;
    let t16043 = t7254 * t3350;
    (t14366, t15093, t15280, t16043)
}
