//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2228;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta497(t1071: f64, t1647: f64, t1695: f64, t3059: f64, t1079: f64, t1651: f64, t3325: f64, t1076: f64, t1097: f64, t11195: f64, t16312: f64, t16314: f64, t16318: f64, t16322: f64, t16328: f64, t16333: f64, t1696: f64, t3052: f64, t3058: f64, t3067: f64, t3271: f64, t3326: f64, t4752: f64, t4778: f64, t4935: f64, t5016: f64, t995: f64, t378: f64, t4743: f64, t1678: f64, t989: f64, t15654: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16340, t16343, t16344, t16352, t16355) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2228(t1071, t1647, t1695, t3059, t1079, t1651, t3325, t1076, t1097, t11195, t16312, t16314, t16318, t16322, t16328, t16333, t1696, t3052, t3058, t3067, t3271, t3326, t4752, t4778, t4935, t5016, t995);
        let (t16362, t16371, t16374) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2229(t378, t4743, t1678, t989, t15654);
    (t16340, t16343, t16344, t16352, t16355, t16362, t16371, t16374)
}
