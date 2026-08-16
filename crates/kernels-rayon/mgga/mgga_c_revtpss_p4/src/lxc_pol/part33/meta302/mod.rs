//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1295;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta302(t10073: f64, t4089: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64, t557: f64, t1429: f64, t9292: f64, t4096: f64, t9285: f64, t1398: f64, t215: f64, t268: f64, t543: f64, t4101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10074, t10098, t10102, t10109, t10111) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1294(t10073, t4089, t1432, t2470, t4107, t1433, t9288, t136, t1419, t2457, t3964, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1295(t10111, t1428, t22, t2452);
        let (t10117, t10126, t10129, t10137) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1296(t10115, t557, t1429, t9292, t3964, t4096, t9285, t1398, t215, t268, t543, t4101);
    (t10074, t10098, t10102, t10109, t10111, t10114, t10115, t10117, t10126, t10129, t10137)
}
