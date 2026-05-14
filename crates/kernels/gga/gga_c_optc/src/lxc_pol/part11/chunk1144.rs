//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1144/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1144<F: Float>(t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t56978: F, t56981: F, t56984: F, t25: F, t56740: F, t794: F) -> (F, F) {
    let t56986 = -0.79724444444444444446e0 * t39411 - 0.5314962962962962963e0 * t39413 + 0.15944888888888888889e1 * t39418 + 0.79724444444444444444e0 * t49240 - 0.23917333333333333333e1 * t49242 - 0.13145066666666666666e1 * t49271 + 0.21908444444444444444e0 * t49273 + 0.71752000000000000001e1 * t56966 - 0.19931111111111111111e1 * t56969 - 0.10954222222222222222e0 * t56972 - 0.21908444444444444444e0 * t56975 - 0.107628e2 * t56978 + 0.23917333333333333333e1 * t56981 - 0.79724444444444444444e0 * t56984;
    let t56988 = t25 * t794 * t56740;
    (t56986, t56988)
}
