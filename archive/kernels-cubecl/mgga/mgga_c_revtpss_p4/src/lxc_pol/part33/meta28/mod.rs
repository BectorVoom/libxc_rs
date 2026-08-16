//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta28 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk200;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk201;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk202;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk203;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta28<F: Float>(t213: F, t547: F, t531: F, t241: F, t247: F, t217: F, t535: F, t225: F, t546: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t548, t549, t550) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk200::<F>(t213, t547, t531);
        let (t552, t555) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk201::<F>(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk202::<F>(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk203::<F>(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk204::<F>(t556, t561, t213);
    (t548, t549, t550, t552, t555, t556, t557, t560, t561, t562, t565, t566)
}
