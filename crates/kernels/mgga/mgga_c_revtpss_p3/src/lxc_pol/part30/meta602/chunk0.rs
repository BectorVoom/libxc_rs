//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2063/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2063<F: Float>(t13085: F, t7624: F, t13017: F, t7607: F, t12901: F, t26844: F, t13014: F, t12998: F, t26866: F, t3746: F, t12773: F, t26867: F) -> (F, F, F, F, F, F, F) {
    let t97200 = t7624 * t13085;
    let t97204 = t7607 * t13017;
    let t97218 = t26844 * t12901;
    let t97220 = t7607 * t13014;
    let t97222 = t7607 * t12998;
    let t97232 = t3746 * t26866;
    let t97239 = t26867 * t12773;
    (t97200, t97204, t97218, t97220, t97222, t97232, t97239)
}
