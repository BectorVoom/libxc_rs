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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1026<F: Float>(t68287: F, t68292: F, t68297: F, t68301: F, t68305: F, t68310: F, t68312: F, t68315: F, t68319: F, t68322: F, t68326: F, t68330: F, t68332: F, t68334: F, t68336: F, t20292: F, t2258: F, t12305: F, t128: F, t2251: F, t43776: F, t5819: F, t3360: F, t68324: F, t68328: F, t13312: F, t5046: F, t51957: F, t56250: F, t60927: F, t56254: F, t56246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t68338 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3590::<F>(t68287, t68292, t68297, t68301, t68305, t68310, t68312, t68315, t68319, t68322, t68326, t68330, t68332, t68334, t68336);
        let (t68340, t68342) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3591::<F>(t20292, t2258, t12305, t128);
        let (t68345, t68347) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3592::<F>(t2251, t43776, t5819, t12305, t128);
        let t68350 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3593::<F>(t128, t3360, t68324);
        let t68353 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3594::<F>(t128, t3360, t68328);
        let (t68355, t68357) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3595::<F>(t13312, t5046, t128, t3360);
        let t68360 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3596::<F>(t51957, t56250, t60927);
        let t68363 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3597::<F>(t51957, t56254, t60927);
        let t68366 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3598::<F>(t51957, t56246, t60927);
    (t68338, t68340, t68342, t68345, t68347, t68350, t68353, t68355, t68357, t68360, t68363, t68366)
}
