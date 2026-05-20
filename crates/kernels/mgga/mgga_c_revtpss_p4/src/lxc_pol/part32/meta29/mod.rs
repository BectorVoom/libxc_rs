//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta29 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk197;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk198;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk199;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk200;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta29<F: Float>(t546: F, t555: F, t213: F, t556: F, t149: F, t198: F, t522: F, t524: F, t532: F, t118: F, t508: F, t511: F, param_d: F, t116: F, t117: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk197::<F>(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk198::<F>(t556, t561, t213);
        let t569 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk199::<F>(t149, t198, t522, t524, t532, t566);
        let (t571, t572) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk200::<F>(t118, t508, t511, t569, param_d);
        let t573 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk201::<F>(t116, t117);
    (t557, t560, t561, t562, t565, t566, t569, t571, t572, t573)
}
