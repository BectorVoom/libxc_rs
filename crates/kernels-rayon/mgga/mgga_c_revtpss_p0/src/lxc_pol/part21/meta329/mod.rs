//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1629;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1630;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1631;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1632;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1633;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta329(t11200: f64, t378: f64, t3059: f64, t999: f64, t996: f64, t3325: f64, t1079: f64, t3043: f64, t3042: f64, t993: f64, t1000: f64, t1076: f64, t1097: f64, t11123: f64, t11128: f64, t11174: f64, t11178: f64, t11184: f64, t11187: f64, t11190: f64, t11195: f64, t3047: f64, t3052: f64, t3060: f64, t3076: f64, t3261: f64, t3326: f64, t989: f64, t995: f64, t1071: f64, t3056: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11201, t11202) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1629(t11200, t378, t3059, t999);
        let (t11203, t11206, t11207, t11210, t11213) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1630(t11202, t996, t3325, t999, t1079, t3043, t378, t3042, t993);
        let t11214 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1631(t11213, t378);
        let t11217 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1632(t1000, t1076, t1097, t11123, t11128, t11174, t11178, t11184, t11187, t11190, t11195, t11201, t11203, t11207, t11210, t11214, t3047, t3052, t3060, t3076, t3261, t3326, t989, t995);
        let (t11220, t11223) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1633(t1071, t989, t3056, t988);
        let t11224 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1634(t11223, t378);
    (t11201, t11202, t11203, t11206, t11207, t11210, t11213, t11214, t11217, t11220, t11223, t11224)
}
