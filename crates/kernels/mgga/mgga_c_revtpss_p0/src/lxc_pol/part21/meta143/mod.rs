//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk921;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk922;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk923;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk924;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk925;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta143<F: Float>(t3357: F, t3358: F, t3365: F, t3370: F, t3374: F, t422: F, t1126: F, t1130: F, t1151: F, t1129: F, t418: F, t408: F, t1149: F, t1150: F, t406: F, t409: F, t1134: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3376, t3378, t3379) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk921::<F>(t3357, t3358, t3365, t3370, t3374, t422, t1126, t1130);
        let (t3381, t3382, t3383) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk922::<F>(t1151, t3379, t1129, t418);
        let t3384 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk923::<F>(t3383, t408);
        let t3385 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk924::<F>(t1149);
        let (t3386, t3388, t3390) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk925::<F>(t1150, t3385, t3384, t406, t409);
        let t3391 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk926::<F>(t1134);
    (t3376, t3378, t3379, t3381, t3382, t3383, t3384, t3385, t3386, t3388, t3390, t3391)
}
