//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 206/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk206<F: Float>(t606: F, t633: F, t637: F, t77: F, t608: F, t628: F, t71: F, t85: F) -> (F, F) {
    let t640 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t606 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t606;
    let t641 = t77 * t640;
    let t644 = -t608 * t85 / F::cast_from(12.0_f64) + t628 * t85 / F::cast_from(24.0_f64) + t71 * t641 / F::cast_from(24.0_f64);
    (t641, t644)
}
