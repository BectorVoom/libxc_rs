//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2049;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2050;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta609(t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t25240: f64, t3964: f64, t5617: f64, t27857: f64, t689: f64, t25904: f64, t786: f64, t97961: f64, t7286: f64, t2439: f64, t7925: f64, t94391: f64, t94383: f64, t25878: f64, t98028: f64, t94771: f64, t97814: f64, t1903: f64, t25931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98270, t98282, t98285, t98303, t98305, t98308) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2049(t26004, t5690, t13951, t2018, t807, t25240, t3964, t5617, t27857, t689, t25904, t786, t97961);
        let (t98310, t98312, t98314, t98333, t98338, t98340) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2050(t7286, t98308, t2439, t7925, t94391, t94383, t25878, t98028, t94771, t97814, t1903, t25931);
    (t98270, t98282, t98285, t98303, t98305, t98310, t98312, t98314, t98333, t98338, t98340)
}
