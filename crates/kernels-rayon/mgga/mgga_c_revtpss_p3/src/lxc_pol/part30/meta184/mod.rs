//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk922;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk923;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk924;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta184(t136: f64, t1413: f64, t1353: f64, t221: f64, t3978: f64, t247: f64, t2682: f64, t550: f64, t548: f64, t1408: f64, t820: f64, t843: f64, t1416: f64, t1386: f64, t240: f64, t1398: f64, t543: f64, t2661: f64, t1384: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3979, t3981, t3982, t3985, t3987, t3989) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk922(t136, t1413, t1353, t221, t3978, t247, t2682, t550, t548, t1408, t820, t843);
        let (t3990, t3992) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk923(t1416, t3989, t1386, t240);
        let t3994 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk924(t1398, t543, t550);
        let (t3995, t3996, t3999) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk925(t3992, t3994, t2661, t1384, t544);
    (t3979, t3981, t3982, t3985, t3987, t3989, t3990, t3992, t3994, t3995, t3996, t3999)
}
