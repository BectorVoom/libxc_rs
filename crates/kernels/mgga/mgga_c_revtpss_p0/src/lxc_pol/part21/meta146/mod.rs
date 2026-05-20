//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta146 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk935;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk936;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk937;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk938;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta146<F: Float>(t421: F, t3385: F, t3433: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1156: F, t1160: F, t1159: F, t431: F, t426: F, t1168: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3434, t3435) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk935::<F>(t421);
        let (t3436, t3438, t3439, t3444, t3447) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk936::<F>(t3385, t3435, t3433, t3356, t3358, t3365, t3370, t3374, t1156, t1160);
        let (t3450, t3451) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk937::<F>(t1159, t431);
        let t3452 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk938::<F>(t3451, t426);
        let t3453 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk939::<F>(t1168);
    (t3434, t3435, t3436, t3438, t3439, t3444, t3447, t3450, t3451, t3452, t3453)
}
