//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3233/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3233<F: Float>(t22648: F, t602: F, t13368: F, t13371: F, t13378: F, t13381: F, t18281: F, t19680: F, t21784: F, t21794: F, t22671: F, t22688: F, t2299: F, t2306: F, t4186: F, t4227: F, t4232: F, t46001: F, t46014: F, t5825: F, t606: F, t633: F, t637: F, t76397: F) -> (F, F) {
    let t85037 = t22648 * t602;
    let t85125 = F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46001 * t22688 * t606 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t21784 * t4186 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t13368 * t19680 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t13371 * t5825 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4227 * t18281 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2299 * t22671 * t606 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t76397 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46014 * t22688 * t606 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t21794 * t4186 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t13378 * t19680 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t13381 * t5825 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t4232 * t18281 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t2306 * t22671 * t606 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t76397;
    (t85037, t85125)
}
