//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta843 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2976;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta843(t13792: f64, t48863: f64, t49137: f64, t13920: f64, t2661: f64, t3992: f64, t543: f64, t550: f64, t1398: f64, t5658: f64, t10073: f64, t14124: f64, t5760: f64, t9292: f64, t10069: f64, t14207: f64, t40921: f64, t5737: f64, t225: f64, t2453: f64, t136: f64, t137: f64, t14140: f64, t2438: f64, t4003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49139, t49144, t49146, t49167) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2976(t13792, t48863, t49137, t13920, t2661, t3992, t543, t550, t1398, t5658, t10073, t14124);
        let (t49172, t49176, t49178, t49180, t49186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2977(t5760, t9292, t10069, t14207, t40921, t5737, t225, t2453, t136, t137, t1398, t14140, t2438, t4003);
    (t49139, t49144, t49146, t49167, t49172, t49176, t49178, t49180, t49186)
}
