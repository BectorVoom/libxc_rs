//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta886 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta886(t15199: f64, t698: f64, t2852: f64, t373: f64, t2439: f64, t4628: f64, t1606: f64, t9303: f64, t11387: f64, t4631: f64, t15513: f64, t914: f64, t2923: f64, t4587: f64, t11384: f64, t1596: f64, t11466: f64, t300: f64, t11452: f64, t4669: f64, t11450: f64, t1621: f64, t11507: f64, t1633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52065, t52110, t52126, t52128, t52163, t52214) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072(t15199, t698, t2852, t373, t2439, t4628, t1606, t9303, t11387, t4631, t15513, t914);
        let (t52219, t52224, t52238, t52264, t52320, t52370) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073(t2923, t4587, t11384, t1596, t11466, t300, t11452, t4669, t11450, t1621, t11507, t1633);
    (t52065, t52110, t52126, t52128, t52163, t52214, t52219, t52224, t52238, t52264, t52320, t52370)
}
