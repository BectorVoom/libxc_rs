//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2971/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2971<F: Float>(t13845: F, t1872: F, t4004: F, t9818: F, t1873: F, t46651: F, t1399: F, t5689: F, t9816: F, t13847: F, t13848: F, t3924: F) -> (F, F, F, F) {
    let t49024 = t13845 * t9818 * t1872 * t4004;
    let t49030 = t46651 * t1873;
    let t49049 = t9816 * t9818 * t5689 * t1399;
    let t49053 = t9816 * t13847 * t13848 * t3924;
    (t49024, t49030, t49049, t49053)
}
