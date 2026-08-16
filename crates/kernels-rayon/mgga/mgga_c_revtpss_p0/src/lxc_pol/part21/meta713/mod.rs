//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta713 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2546;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta713(t46740: f64, t9821: f64, t13999: f64, t9842: f64, t9828: f64, t9962: f64, t124: f64, t3923: f64, t3938: f64, t9816: f64, t9818: f64, t9769: f64, t9793: f64, t9794: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t4006: f64, t1399: f64, t2661: f64, t3992: f64, t9929: f64, t1412: f64, t4056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46741, t46747, t46749, t46751, t46754, t46757) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2546(t46740, t9821, t13999, t9842, t9828, t9962, t124, t3923, t3938, t9816, t9818, t9769, t9793, t9794);
        let (t46760, t46767, t46771, t46776) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2547(t1376, t40757, t2681, t4000, t820, t4006, t1399, t2661, t3992, t9929, t1412, t3938, t4056);
    (t46741, t46747, t46749, t46751, t46754, t46757, t46760, t46767, t46771, t46776)
}
