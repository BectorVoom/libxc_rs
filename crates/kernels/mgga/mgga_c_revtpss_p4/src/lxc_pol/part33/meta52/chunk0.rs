//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 341/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk341<F: Float>(t1015: F, t606: F, t1012: F, t225: F, t989: F, t366: F, t994: F) -> (F, F, F, F, F) {
    let t1016 = t1015 * t606;
    let t1017 = t1012 * t1016;
    let t1020 = t989 * t225;
    let t1021 = t1020 * t366;
    let t1024 = t994 * t225;
    (t1016, t1017, t1020, t1021, t1024)
}
