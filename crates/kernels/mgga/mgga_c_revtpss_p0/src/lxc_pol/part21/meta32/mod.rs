//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta32 (260520-c91 hierarchical CSE).
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
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk239;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk240;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk241;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk242;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk243;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk244;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk245;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk246;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk247;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk248;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta32<F: Float>(t45: F, t78: F, t57: F, t81: F, t606: F, t77: F, t608: F, t628: F, t71: F, t85: F, t5: F, t599: F, t603: F, t91: F, t117: F, t116: F, t94: F, t112: F, t625: F, t111: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t631 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk239::<F>(t45);
        let t633 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk240::<F>(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk241::<F>(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk242::<F>(t635, t81);
        let t641 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk243::<F>(t606, t633, t637, t77);
        let t644 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk244::<F>(t608, t628, t641, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk245::<F>(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk246::<F>(t117, t648);
        let t651 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk247::<F>(t116, t94);
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk248::<F>(t112, t625, t111);
    (t631, t633, t635, t637, t641, t644, t648, t649, t651, t653, t654, t655)
}
