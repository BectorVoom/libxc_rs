//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 702/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk702<F: Float>(t6900: F, t6959: F, t2096: F, t669: F, t2105: F, t664: F, t668: F, t145: F, t2107: F, t708: F, t2189: F, t2126: F, t6786: F, t128: F, t2155: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6960 = t6900 + t6959;
    let t6964 = t2096 * t669;
    let t6968 = t664 * t2105;
    let t6975 = t668 * t668;
    let t6976 = 1.0 / t6975;
    let t6977 = t145 * t6976;
    let t6978 = t2107 * t708;
    let t6982 = t2105 * t708;
    let t6983 = t6982 * t2189;
    let t6986 = t2126 * t6786;
    let t6990 = 1.0 / t2155 / t128;
    let t6991 = t6990 * t131;
    (t6960, t6964, t6968, t6975, t6976, t6977, t6978, t6982, t6983, t6986, t6990, t6991)
}
