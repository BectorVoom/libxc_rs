//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1089/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1089<F: Float>(t11375: F, t1185: F, t13924: F, t50995: F, t51053: F, t51675: F, t53134: F, t53140: F, t53152: F, t53155: F, t53158: F, t53166: F, t53170: F, t53174: F, t53177: F, t53179: F, t53182: F, t8776: F, t9697: F) -> (F,) {
    let t53184 = t53134 / 48.0 + 7.0 / 288.0 * t50995 - t53140 / 384.0 + t8776 * t1185 * t13924 / 32.0 - t9697 * t1185 * t51053 / 32.0 - t11375 * t51675 / 48.0 + t53152 / 384.0 - t53155 - t53158 / 96.0 - t53166 / 384.0 + t53170 / 384.0 + t53174 / 768.0 - t53177 - t53179 + 5.0 / 768.0 * t53182;
    (t53184,)
}
