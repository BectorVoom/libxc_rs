//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2156;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta520(t3057: f64, t379: f64, t1078: f64, t1651: f64, t3066: f64, t1695: f64, t3325: f64, t3269: f64, t3270: f64, t11121: f64, t5015: f64, t999: f64, t1079: f64, t342: f64, t4930: f64, t1071: f64, t1647: f64, t3059: f64, t1076: f64, t1097: f64, t11195: f64, t1696: f64, t3052: f64, t3058: f64, t3067: f64, t3271: f64, t3326: f64, t4752: f64, t4778: f64, t4935: f64, t5016: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16312, t16313, t16314, t16318, t16322, t16327) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2156(t3057, t379, t1078, t1651, t3066, t1695, t3325, t3269, t3270, t11121, t5015, t999);
        let (t16328, t16333, t16340, t16344, t16352, t16355) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2157(t1079, t16327, t342, t4930, t1071, t1647, t1695, t3059, t1651, t3325, t1076, t1097, t11195, t16312, t16314, t16318, t16322, t1696, t3052, t3058, t3067, t3271, t3326, t4752, t4778, t4935, t5016, t995);
    (t16312, t16313, t16314, t16318, t16322, t16328, t16333, t16340, t16344, t16352, t16355)
}
