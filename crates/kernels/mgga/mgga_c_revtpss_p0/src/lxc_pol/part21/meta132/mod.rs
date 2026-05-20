//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk852;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk853;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk854;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk855;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta132<F: Float>(t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F, t1045: F, t373: F, t1042: F, t1031: F, t196: F, t342: F, t1034: F, t358: F, t360: F, t368: F, t335: F) -> (F, F, F, F, F, F, F, F, F) {
        let t3133 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk852::<F>(t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036);
        let (t3135, t3136) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk853::<F>(t1045, t3133, t373, t1042);
        let t3140 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk854::<F>(t1031, t196);
        let (t3141, t3143, t3144, t3145) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk855::<F>(t3140, t342, t1034, t358, t360, t368);
        let t3147 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk856::<F>(t3145, t335);
    (t3133, t3135, t3136, t3140, t3141, t3143, t3144, t3145, t3147)
}
