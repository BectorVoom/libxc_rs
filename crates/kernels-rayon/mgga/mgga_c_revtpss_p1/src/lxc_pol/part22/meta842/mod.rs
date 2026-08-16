//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta842(t14230: f64, t46802: f64, t49068: f64, t46888: f64, t48908: f64, t1398: f64, t5591: f64, t13946: f64, t9962: f64, t1413: f64, t46835: f64, t48694: f64, t13775: f64, t9793: f64, t9794: f64, t5690: f64, t9741: f64, t14016: f64, t46691: f64, t14020: f64, t3957: f64, t2659: f64, t5744: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49103, t49105, t49107, t49118, t49121) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2974(t14230, t46802, t49068, t46888, t48908, t1398, t5591, t13946, t9962, t1413, t46835, t48694);
        let (t49124, t49126, t49128, t49134, t49137) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2975(t13775, t9793, t9794, t5690, t9741, t14016, t46691, t14020, t3957, t2659, t5744, t816);
    (t49103, t49105, t49107, t49118, t49121, t49124, t49126, t49128, t49134, t49137)
}
