//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2546;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta713<F: Float>(t46740: F, t9821: F, t13999: F, t9842: F, t9828: F, t9962: F, t124: F, t3923: F, t3938: F, t9816: F, t9818: F, t9769: F, t9793: F, t9794: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t4006: F, t1399: F, t2661: F, t3992: F, t9929: F, t1412: F, t4056: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46741, t46747, t46749, t46751, t46754, t46757) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2546::<F>(t46740, t9821, t13999, t9842, t9828, t9962, t124, t3923, t3938, t9816, t9818, t9769, t9793, t9794);
        let (t46760, t46767, t46771, t46776) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2547::<F>(t1376, t40757, t2681, t4000, t820, t4006, t1399, t2661, t3992, t9929, t1412, t3938, t4056);
    (t46741, t46747, t46749, t46751, t46754, t46757, t46760, t46767, t46771, t46776)
}
