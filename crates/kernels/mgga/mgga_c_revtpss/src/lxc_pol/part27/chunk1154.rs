//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1154/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1154<F: Float>(t13017: F, t7607: F, t13032: F, t26843: F, t13036: F, t13038: F, t13040: F, t26842: F, t12901: F, t26844: F, t13014: F, t12998: F, t26866: F, t3746: F, t12773: F, t26867: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97204 = t7607 * t13017;
    let t97206 = t13032 * t26843;
    let t97211 = t13036 * t13038 * sigma2 * t13040;
    let t97215 = t13036 * t26842 * t13040;
    let t97218 = t26844 * t12901;
    let t97220 = t7607 * t13014;
    let t97222 = t7607 * t12998;
    let t97232 = t3746 * t26866;
    let t97239 = t26867 * t12773;
    (t97204, t97206, t97211, t97215, t97218, t97220, t97222, t97232, t97239)
}
