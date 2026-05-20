//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1150;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1151;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta285<F: Float>(t12032: F, t225: F, t385: F, t3270: F, t999: F, t3269: F, t11804: F, t996: F, t1035: F, t11239: F, t342: F, t11247: F, t378: F, t3145: F, t334: F, t11249: F) -> (F, F, F, F, F, F, F, F) {
        let (t12034, t12040, t12043, t12046, t12047, t12048) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1149::<F>(t12032, t225, t385, t3270, t999, t3269, t11804, t996, t1035, t11239, t342, t11247, t378);
        let t12050 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1150::<F>(t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1151::<F>(t11249, t12050);
    (t12034, t12040, t12043, t12046, t12047, t12048, t12050, t12051)
}
