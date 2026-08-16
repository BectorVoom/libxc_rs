//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta841(t13910: f64, t808: f64, t9736: f64, t14026: f64, t9744: f64, t13821: f64, t13999: f64, t13716: f64, t1413: f64, t547: f64, t807: f64, t550: f64, t9794: f64, t14224: f64, t9793: f64, t13928: f64, t9962: f64, t13800: f64, t46670: f64, t3964: f64, t5617: f64, t9732: f64, t136: f64, t216: f64, t9747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49056, t49058, t49062, t49066, t49068) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2972(t13910, t808, t9736, t14026, t9744, t13821, t13999, t13716, t1413, t547, t807, t550, t9794);
        let (t49070, t49085, t49087, t49090, t49093) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2973(t14224, t49068, t9793, t13928, t9962, t13800, t46670, t3964, t5617, t9732, t136, t216, t9747);
    (t49056, t49058, t49062, t49066, t49068, t49070, t49085, t49087, t49090, t49093)
}
