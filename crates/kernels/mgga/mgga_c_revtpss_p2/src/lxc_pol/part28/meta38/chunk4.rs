//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 254/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk254<F: Float>(t123: F, t173: F, t186: F, t676: F, t679: F, t704: F, t724: F, t731: F, t739: F, t746: F) -> F {
    let t749 = F::cast_from(0.53237641966666666666e-3_f64) * t123 * t676 * t173 + F::new(1.0) * t724 * t731 - t679 - t704 + F::cast_from(0.18311447306006545054e-3_f64) * t123 * t676 * t186 + F::cast_from(0.5848223622634646207e0_f64) * t739 * t746;
    t749
}
