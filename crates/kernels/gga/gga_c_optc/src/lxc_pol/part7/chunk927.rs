//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 927/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk927<F: Float>(t43: F, t1885: F, t1891: F, t1933: F, t22014: F, t22015: F, t22021: F, t22028: F, t607: F, t6533: F, t6537: F, t6541: F, t2854: F, t52: F, t1897: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t22032 = piecewise3(t44, 0.0, -56.0 / 81.0 * t22014 * t22015 + 16.0 / 9.0 * t6533 * t1885 * t1891 - 2.0 / 3.0 * t1933 * t22021 - 8.0 / 9.0 * t6537 * t6541 + 2.0 / 3.0 * t607 * t22028);
    let t22034 = 1.0 / t52 / t2854;
    let t22035 = t1897 * t1897;
    (t22032, t22034, t22035)
}
