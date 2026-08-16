//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 551/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk551<F: Float>(t45: F, t57: F, t2251: F, t2258: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F) -> F {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2422 = piecewise3::<F>(t151, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t80 * t2251 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t766 * t2258);
    let t2428 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t83 * t2251 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t2258);
    let t2430 = t2422 / F::cast_from(2.0_f64) + t2428 / F::cast_from(2.0_f64);
    t2430
}
