//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta532(t25386: f64, t95536: f64, t92840: f64, t26518: f64, t9285: f64, t25299: f64, t7407: f64, t92890: f64, t2061: f64, t22: f64, t25402: f64, t93140: f64, t25310: f64, t26506: f64, t26485: f64, t93364: f64, t2829: f64, t689: f64, t7384: f64, t2439: f64, t7398: f64, t780: f64, t785: f64, t93134: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t95537, t95538, t95540, t95542, t95543, t95546, t95548) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1862(t25386, t95536, t92840, t26518, t9285, t25299, t7407, t92890, t2061, t22, t25402, t93140);
        let (t95551, t95553, t95556, t95562, t95567) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1863(t25310, t26506, t26485, t93364, t2829, t689, t7384, t2439, t7398, t780, t785, t93134, t95546);
    (t95537, t95538, t95540, t95542, t95543, t95548, t95551, t95553, t95556, t95562, t95567)
}
