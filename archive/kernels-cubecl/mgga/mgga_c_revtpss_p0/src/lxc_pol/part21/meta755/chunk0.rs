//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2647/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647<F: Float>(t1353: F, t1883: F, t46825: F, t9793: F, t13848: F, t9810: F, t9816: F, t9818: F, t1408: F, t241: F, t820: F, t2482: F, t814: F, t9991: F) -> (F, F, F, F, F) {
    let t48698 = t1883 * t1353;
    let t48700 = t9793 * t46825 * t48698;
    let t48709 = t9816 * t9818 * t13848 * t9810;
    let t48712 = t820 * t1408 * t241;
    let t48731 = t2482 * t9991 * t814;
    (t48698, t48700, t48709, t48712, t48731)
}
