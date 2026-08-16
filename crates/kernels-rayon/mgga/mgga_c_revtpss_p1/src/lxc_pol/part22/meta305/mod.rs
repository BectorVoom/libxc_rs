//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1741;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1742;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta305(t10115: f64, t557: f64, t10024: f64, t268: f64, t543: f64, t4101: f64, t1429: f64, t9292: f64, t3964: f64, t4096: f64, t9285: f64, t1385: f64, t4066: f64, t1398: f64, t215: f64, t2453: f64, t4100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10117, t10119, t10120, t10126, t10129, t10130) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1741(t10115, t557, t10024, t268, t543, t4101, t1429, t9292, t3964, t4096, t9285, t1385, t4066);
        let (t10136, t10137, t10139) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1742(t1398, t215, t268, t543, t4101, t2453, t4100);
    (t10117, t10119, t10120, t10126, t10129, t10130, t10136, t10137, t10139)
}
