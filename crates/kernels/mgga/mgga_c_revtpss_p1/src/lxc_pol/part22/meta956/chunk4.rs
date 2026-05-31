//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3204/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3204<F: Float>(t10389: F, t10398: F, t13312: F, t13368: F, t13378: F, t13396: F, t18281: F, t21784: F, t21789: F, t21794: F, t21799: F, t2251: F, t2258: F, t2299: F, t2306: F, t4227: F, t4232: F, t46001: F, t46014: F, t5819: F, t5825: F, t606: F, t60717: F, t60754: F, t633: F, t637: F) -> F {
    let t60778 = F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46001 * t5819 * t2251 - F::cast_from(1120.0_f64) / F::cast_from(27.0_f64) * t13368 * t13396 - F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t21784 * t2258 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t2299 * t60717 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4227 * t13312 - F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t10389 * t5825 * t2251 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t2299 * t18281 * t606 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t21789 * t2258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t60754 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46014 * t5819 * t2251 + F::cast_from(1120.0_f64) / F::cast_from(27.0_f64) * t13378 * t13396 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t21794 * t2258 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t2306 * t60717 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4232 * t13312 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t10398 * t5825 * t2251 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t2306 * t18281 * t606 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t21799 * t2258 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t60754;
    t60778
}
