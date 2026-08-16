//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1025 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3586;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3587;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3588;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1025(t20267: f64, t698: f64, t1145: f64, t141: f64, t68273: f64, t2258: f64, t6421: f64, t68269: f64, t20297: f64, t3417: f64, t20292: f64, t2251: f64, t20314: f64, t689: f64, t20303: f64, t20299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68312, t68315, t68317, t68319, t68322, t68324, t68326, t68328) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3586(t20267, t698, t1145, t141, t68273, t2258, t6421, t68269, t20297, t3417, t20292, t2251);
        let (t68330, t68332) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3587(t141, t3417, t68328, t20314, t689);
        let t68334 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3588(t20303, t689);
        let t68336 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3589(t20299, t689);
    (t68312, t68315, t68317, t68319, t68322, t68324, t68326, t68328, t68330, t68332, t68334, t68336)
}
