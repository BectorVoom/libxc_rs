//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 247/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk247<F: Float>(t902: F, t340: F, t338: F) -> (F, F, F, F) {
    let t986 = F::cast_from(0.83333333333333333333e-2_f64) * t902;
    let t992 = t340 * t340;
    let t993 = F::cast_from(1.0_f64) / t992;
    let t994 = t338 * t993;
    (t986, t992, t993, t994)
}
