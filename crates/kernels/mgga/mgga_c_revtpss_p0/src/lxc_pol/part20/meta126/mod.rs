//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk720;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk721;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk722;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta126<F: Float>(t421: F, t3385: F, t3433: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1156: F, t1160: F, t1159: F, t431: F, t426: F, t1168: F, t1169: F, t3413: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3434, t3435) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk720::<F>(t421);
        let (t3436, t3438, t3444, t3447, t3450) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk721::<F>(t3385, t3435, t3433, t3356, t3358, t3365, t3370, t3374, t1156, t1160, t1159, t431);
        let (t3451, t3452, t3453) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk722::<F>(t3450, t426, t1168);
        let (t3454, t3471) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk723::<F>(t1169, t3453, t3356, t3413, t3358, t3365, t3370, t3374, t3392, t3400, t3408, t3410, t3415, t3419, t3422, t3425);
    (t3434, t3435, t3436, t3438, t3444, t3447, t3450, t3451, t3452, t3453, t3454, t3471)
}
