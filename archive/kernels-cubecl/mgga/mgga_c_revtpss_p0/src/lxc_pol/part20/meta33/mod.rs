//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta33 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk243;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk244;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk245;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk246;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta33<F: Float>(t123: F, t147: F, t676: F, t143: F, t130: F, t131: F, t72: F, t122: F, t125: F, t675: F) -> (F, F, F, F, F, F, F, F, F) {
        let t679 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk243::<F>(t123, t147, t676);
        let (t680, t681, t682, t684, t685) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk244::<F>(t143, t130, t131, t72, t122, t125);
        let t686 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk245::<F>(t675, t685);
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk246::<F>(t684, t686, t123, t676);
    (t679, t680, t681, t682, t684, t685, t686, t687, t689)
}
