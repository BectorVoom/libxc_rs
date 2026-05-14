//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 606/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk606<F: Float>(t1595: F, t422: F, t120: F, t358: F, t363: F, t528: F, t7899: F, t72: F, t123: F, t532: F, t7911: F, t126: F, t1655: F, t535: F, t122: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8964 = t422 * t1595;
    let t8965 = t120 * t358;
    let t8966 = t8965 * t363;
    let t8967 = t8964 * t8966;
    let t8971 = t7899 * t528 * t120;
    let t8972 = t72 * t8971;
    let t8977 = t123 / t532 / t7911;
    let t8978 = t7899 * t126;
    let t8981 = t535 * t1655;
    let t8991 = t122 * t122;
    (t8964, t8965, t8966, t8967, t8972, t8977, t8978, t8981, t8991)
}
