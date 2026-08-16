//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1818;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1819;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta479(t3167: f64, t7120: f64, t1033: f64, t3173: f64, t7122: f64, t2269: f64, t343: f64, t136: f64, t1007: f64, t7106: f64, t1968: f64, t3080: f64, t7105: f64, t800: f64, t1017: f64, t1028: f64, t1047: f64, t25490: f64, t25495: f64, t25498: f64, t25500: f64, t25505: f64, t25509: f64, t25512: f64, t25517: f64, t25522: f64, t3097: f64, t3130: f64, t3136: f64, t3157: f64, t3164: f64, t3208: f64, t3220: f64, t348: f64, t7117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25525, t25526) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1818(t3167, t7120, t1033);
        let (t25529, t25531, t25532, t25535, t25538, t25539) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1819(t3173, t7122, t2269, t343, t136, t1007, t7106, t1968, t3080, t7105, t800);
        let t25542 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1820(t1017, t1028, t1047, t25490, t25495, t25498, t25500, t25505, t25509, t25512, t25517, t25522, t25526, t25529, t25532, t25535, t25538, t25539, t3097, t3130, t3136, t3157, t3164, t3208, t3220, t348, t7117, t7122);
    (t25525, t25526, t25529, t25531, t25532, t25535, t25538, t25539, t25542)
}
