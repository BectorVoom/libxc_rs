//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 658/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk658<F: Float>(t3204: F, t366: F, t3059: F, t373: F, t371: F, t372: F, t1024: F, t1053: F) -> (F, F, F, F) {
    let t3205 = t3204 * t366;
    let t3206 = t373 * t3059;
    let t3208 = t371 * t372 * t3206;
    let t3211 = t1024 * t1053;
    (t3205, t3206, t3208, t3211)
}
