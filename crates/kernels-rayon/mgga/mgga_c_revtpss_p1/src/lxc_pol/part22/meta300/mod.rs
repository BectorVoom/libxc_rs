//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1731;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta300(t4021: f64, t9976: f64, t1398: f64, t1412: f64, t3938: f64, t3992: f64, t2661: f64, t1384: f64, t235: f64, t4003: f64, t543: f64, t2482: f64, t27: f64, t4000: f64, t221: f64, t4004: f64, t4019: f64, t1419: f64, t4086: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9977, t9981, t9982, t9989, t9990, t9991, t9994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1731(t4021, t9976, t1398, t1412, t3938, t3992, t2661, t1384, t235, t4003, t543);
        let (t10001, t10003, t10004, t10013, t10014) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1732(t2482, t27, t4000, t221, t4004, t4019, t1419, t4086, t786);
    (t9977, t9981, t9982, t9989, t9990, t9991, t9994, t10001, t10003, t10004, t10013, t10014)
}
