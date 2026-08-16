//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1295;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta308(t4018: f64, t9970: f64, t3930: f64, t4059: f64, t1386: f64, t2482: f64, t596: f64, t4021: f64, t1398: f64, t1412: f64, t3938: f64, t3992: f64, t2661: f64, t1384: f64, t235: f64, t4003: f64, t543: f64, t27: f64, t4000: f64, t221: f64, t4004: f64, t4019: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9971, t9973, t9976, t9977, t9980, t9981) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1295(t4018, t9970, t3930, t4059, t1386, t2482, t596, t4021, t1398, t1412, t3938, t3992);
        let (t9982, t9990, t9991, t9994, t10001, t10003) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1296(t2661, t9981, t1384, t235, t4003, t543, t2482, t27, t4000, t221, t4004, t4019);
    (t9971, t9973, t9976, t9977, t9980, t9982, t9990, t9991, t9994, t10001, t10003)
}
