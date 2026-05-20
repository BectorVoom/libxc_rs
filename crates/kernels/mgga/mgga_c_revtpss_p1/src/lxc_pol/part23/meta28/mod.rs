//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk215;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk216;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk217;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk218;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk219;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk220;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk221;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta28<F: Float>(t549: F, t241: F, t247: F, t217: F, t535: F, t548: F, t225: F, t546: F, t213: F, t149: F, t198: F, t522: F, t524: F, t532: F, t118: F, t508: F, t511: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t550 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk215::<F>(t549);
        let t555 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk216::<F>(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk217::<F>(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk218::<F>(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk219::<F>(t556, t561, t213);
        let t569 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk220::<F>(t149, t198, t522, t524, t532, t566);
        let (t571, t572) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk221::<F>(t118, t508, t511, t569, param_d);
    (t550, t555, t556, t557, t560, t561, t562, t565, t566, t569, t571, t572)
}
