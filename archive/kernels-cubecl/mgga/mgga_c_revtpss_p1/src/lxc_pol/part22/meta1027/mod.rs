//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1027 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3599;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3600;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3601;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3602;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1027<F: Float>(t20340: F, t698: F, t20377: F, t5079: F, t3407: F, t43911: F, t56176: F, t56183: F, t56185: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t68363: F, t68366: F, t20289: F, t689: F, t1121: F, t60754: F, t1120: F, t128: F, t3362: F, t60717: F, t3360: F, t2435: F, t6426: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t68368, t68370, t68372, t68373, t68379) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3599::<F>(t20340, t698, t20377, t5079, t3407, t43911, t56176, t56183, t56185, t68342, t68347, t68350, t68353, t68357, t68360, t68363, t68366);
        let t68389 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3600::<F>(t20289, t689);
        let (t68391, t68393) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3601::<F>(t1121, t60754, t1120, t128);
        let (t68395, t68397) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3602::<F>(t3362, t60717, t128, t3360);
        let t68399 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3603::<F>(t2435, t6426);
    (t68368, t68370, t68372, t68373, t68379, t68389, t68391, t68393, t68395, t68397, t68399)
}
