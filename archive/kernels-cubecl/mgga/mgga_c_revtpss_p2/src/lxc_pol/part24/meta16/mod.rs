//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta16 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk129;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk130;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk131;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk132;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta16<F: Float>(t275: F, t291: F, t153: F, t159: F, t162: F, zeta_threshold: F, t273: F, t276: F, t279: F, t285: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t293, t300) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk129::<F>(t275, t291, t153, t159, t162, zeta_threshold);
        let t302 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk130::<F>(t273);
        let (t307, t310, t311) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk131::<F>(t273, t276, t279, t285);
        let t315 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk132::<F>(t273);
        let (t320, t323, t324) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk133::<F>(t273, t276, t279, t285);
    (t293, t300, t302, t307, t310, t311, t315, t320, t323, t324)
}
