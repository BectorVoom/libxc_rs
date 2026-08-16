//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1027;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta281(t10059: f64, t4086: f64, t543: f64, t2782: f64, t123: f64, t212: f64, t2434: f64, t4089: f64, t138: f64, t2438: f64, t785: f64, t1398: f64, t1419: f64, t4056: f64, t555: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64, t4066: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10066, t10069, t10070, t10073, t10074, t10079) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1027(t10059, t4086, t543, t2782, t123, t212, t2434, t4089, t138, t2438, t785, t1398, t1419);
        let (t10080, t10085, t10098, t10102, t10103) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1028(t10079, t2782, t4056, t555, t4086, t543, t1432, t2470, t4107, t1433, t9288, t4066, t72);
    (t10066, t10069, t10070, t10073, t10074, t10080, t10085, t10098, t10102, t10103)
}
