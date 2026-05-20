//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta34 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk258;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk259;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk260;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta34<F: Float>(t143: F, t130: F, t131: F, t72: F, t122: F, t125: F, t675: F, t123: F, t676: F, t128: F, t3: F, t66: F, t124: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t680, t681, t682, t684, t685) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk258::<F>(t143, t130, t131, t72, t122, t125);
        let t686 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk259::<F>(t675, t685);
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk260::<F>(t684, t686, t123, t676);
        let (t692, t693, t696, t697) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk261::<F>(t128, t72, t686, t3, t66, t124);
    (t680, t681, t682, t684, t685, t686, t687, t689, t692, t693, t696, t697)
}
