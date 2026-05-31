//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1129/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1129<F: Float>(t10389: F, t1469: F, t2299: F, t4186: F, t10398: F, t2306: F, t13312: F, t2251: F, t2258: F, t4227: F, t4232: F, t606: F, t633: F, t637: F) -> F {
    let t13368 = t10389 * t1469;
    let t13371 = t2299 * t4186;
    let t13378 = t10398 * t1469;
    let t13381 = t2306 * t4186;
    let t13388 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t13368 * t2251 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t13371 * t606 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4227 * t2258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t13312 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t13378 * t2251 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t13381 * t606 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4232 * t2258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t13312;
    t13388
}
