//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2958/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2958<F: Float>(t13769: F, t808: F, t9736: F, t13952: F, t2689: F, t13784: F, t543: F, t46825: F, t9793: F, t1353: F, t1883: F, t13848: F, t9810: F, t9816: F, t9818: F) -> (F, F, F, F, F, F, F) {
    let t48690 = t9736 * t808 * t13769;
    let t48692 = t2689 * t13952;
    let t48694 = t13784 * t543;
    let t48696 = t9793 * t46825 * t48694;
    let t48698 = t1883 * t1353;
    let t48700 = t9793 * t46825 * t48698;
    let t48709 = t9816 * t9818 * t13848 * t9810;
    (t48690, t48692, t48694, t48696, t48698, t48700, t48709)
}
