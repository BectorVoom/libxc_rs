//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta148 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk781;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk782;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta148<F: Float>(t1026: F, t127: F, t371: F, t1025: F, t3075: F, t373: F, t372: F, t225: F, t3046: F, t366: F) -> (F, F, F, F, F, F) {
        let (t3215, t3216, t3218, t3220, t3223) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk781::<F>(t1026, t127, t371, t1025, t3075, t373, t372, t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk782::<F>(t3223, t366);
    (t3215, t3216, t3218, t3220, t3223, t3224)
}
