//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 432/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk432<F: Float>(t1955: F, t1975: F, t1947: F, t1967: F, t623: F, t627: F, t74: F, t79: F, t81: F, t82: F, t1956: F, t1959: F, t616: F, t618: F, t632: F, t72: F, t85: F) -> (F, F, F) {
    let t1976 = t1975 * t1955;
    let t1987 = -2.0 * t1967 * t1955 * t81 + t623 * t1947 * t81 / 2.0 + t1976 * t81 / 4.0 - 4.0 * t1955 * t82 - t79 * t1955 * t81 - 4.0 * t627 * t1947 - t74 * t1947 * t81;
    let t1990 = -t1956 * t81 / 2.0 + 2.0 * t1959 * t1955 - t618 * t1947 + 2.0 * t1947 * t85 + 4.0 * t616 * t632 + 2.0 * t72 * t1987;
    (t1976, t1987, t1990)
}
