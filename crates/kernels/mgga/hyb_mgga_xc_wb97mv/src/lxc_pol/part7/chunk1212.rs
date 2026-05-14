//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1212/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1212<F: Float>(t21365: F, t21367: F, t21369: F, t21428: F, t24882: F, t24894: F, t24896: F, t24898: F, t24900: F, t24902: F, t24904: F, t24906: F, t10: F, t17: F, t24958: F, t3038: F) -> (F, F) {
    let t29282 = 28.0 / 729.0 * t24882 - 142.0 / 243.0 * t24894 + t21428 + 10.0 / 729.0 * t24896 + 8.0 / 243.0 * t24898 - 2.0 / 81.0 * t24900 - 8.0 / 81.0 * t24902 + 16.0 / 243.0 * t24904 + 2.0 / 243.0 * t24906 + 28.0 / 729.0 * t21365 - 2.0 / 243.0 * t21367 - 4.0 / 729.0 * t21369;
    let t29290 = t24958 * t10 * t3038 * t17;
    (t29282, t29290)
}
