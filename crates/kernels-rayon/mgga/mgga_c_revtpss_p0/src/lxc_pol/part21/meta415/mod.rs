//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1890;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1891;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta415(t13334: f64, t38: f64, t1486: f64, t2251: f64, t2259: f64, t4217: f64, t607: f64, t1471: f64, t1487: f64, t1494: f64, t2252: f64, t2260: f64, t2263: f64, t2312: f64, t4196: f64, t4218: f64, t4238: f64, t608: f64, t641: f64, t85: f64, t10389: f64, t1469: f64, t2299: f64, t4186: f64, t10398: f64, t2306: f64, t13312: f64, t2258: f64, t4227: f64, t4232: f64, t606: f64, t633: f64, t637: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13335, t13340, t13343, t13346, t13363) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1890(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
        let (t13368, t13371, t13378, t13381, t13388) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1891(t10389, t1469, t2299, t4186, t10398, t2306, t13312, t2251, t2258, t4227, t4232, t606, t633, t637);
        let (t13389, t13392) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1892(t13388, t77, t1469, t2258);
    (t13335, t13340, t13343, t13346, t13363, t13368, t13371, t13378, t13381, t13389, t13392)
}
