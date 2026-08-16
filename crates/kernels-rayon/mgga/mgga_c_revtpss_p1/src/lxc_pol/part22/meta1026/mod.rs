//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1026 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3590;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3591;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3592;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3593;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3594;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3595;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3596;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3597;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1026(t68287: f64, t68292: f64, t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68312: f64, t68315: f64, t68319: f64, t68322: f64, t68326: f64, t68330: f64, t68332: f64, t68334: f64, t68336: f64, t20292: f64, t2258: f64, t12305: f64, t128: f64, t2251: f64, t43776: f64, t5819: f64, t3360: f64, t68324: f64, t68328: f64, t13312: f64, t5046: f64, t51957: f64, t56250: f64, t60927: f64, t56254: f64, t56246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68338 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3590(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let (t68340, t68342) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3591(t20292, t2258, t12305, t128);
        let (t68345, t68347) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3592(t2251, t43776, t5819, t12305, t128);
        let t68350 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3593(t128, t3360, t68324);
        let t68353 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3594(t128, t3360, t68328);
        let (t68355, t68357) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3595(t13312, t5046, t128, t3360);
        let t68360 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3596(t51957, t56250, t60927);
        let t68363 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3597(t51957, t56254, t60927);
        let t68366 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3598(t51957, t56246, t60927);
    (t68338, t68340, t68342, t68345, t68347, t68350, t68353, t68355, t68357, t68360, t68363, t68366)
}
