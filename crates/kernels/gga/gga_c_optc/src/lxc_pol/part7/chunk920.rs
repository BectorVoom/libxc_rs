//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 920/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk920<F: Float>(t209: F, t514: F, t535: F, t622: F, t110: F, t1756: F, t1759: F, t6475: F, t6481: F, t6607: F, t758: F, t6529: F, t1909: F, t201: F, t7159: F, t9412: F) -> (F, F, F, F, F, F, F) {
    let t21899 = 0.22161481481481481481e0 * t209 * t622 * t514 * t535;
    let t21903 = 0.28493333333333333334e0 * t209 * t110 * t1756 * t1759;
    let t21907 = 0.4274e0 * t209 * t6481 * t6475;
    let t21911 = t6607 * t758;
    let t21913 = t6529 * t758;
    let t21915 = t1909 * t201;
    let t21920 = t9412 * t7159;
    (t21899, t21903, t21907, t21911, t21913, t21915, t21920)
}
