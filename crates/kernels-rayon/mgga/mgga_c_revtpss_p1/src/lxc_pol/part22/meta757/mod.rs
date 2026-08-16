//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta757 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2835;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta757(t1063: f64, t247: f64, t42778: f64, t906: f64, t11986: f64, t2858: f64, t373: f64, t675: f64, t828: f64, t3115: f64, t3119: f64, t11249: f64, t3151: f64, t3046: f64, t3316: f64, t4891: f64, t11238: f64, t196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42781, t42785, t42792, t42793) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2835(t1063, t247, t42778, t906, t11986, t2858, t373, t675, t828);
        let (t42795, t42804, t42830, t42859) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2836(t3115, t3119, t42793, t11249, t3151, t3046, t3316, t4891, t11238, t196);
    (t42781, t42785, t42792, t42793, t42795, t42804, t42830, t42859)
}
