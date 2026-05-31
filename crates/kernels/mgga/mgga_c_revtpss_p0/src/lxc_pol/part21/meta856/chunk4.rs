//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3251/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3251<F: Float>(t10326: F, t10356: F, t10389: F, t10398: F, t11231: F, t13312: F, t13368: F, t13371: F, t13378: F, t13381: F, t1469: F, t2251: F, t2258: F, t2299: F, t2306: F, t4186: F, t4227: F, t4232: F, t46001: F, t46014: F, t49889: F, t606: F, t633: F, t637: F) -> F {
    let t60479 = F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46001 * t1469 * t10356 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t10389 * t4186 * t2251 - F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t13368 * t11231 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2299 * t13312 * t606 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t13371 * t2258 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4227 * t10326 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t49889 + F::cast_from(3640.0_f64) / F::cast_from(81.0_f64) * t46014 * t1469 * t10356 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t10398 * t4186 * t2251 + F::cast_from(280.0_f64) / F::cast_from(9.0_f64) * t13378 * t11231 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t2306 * t13312 * t606 + F::cast_from(28.0_f64) / F::cast_from(3.0_f64) * t13381 * t2258 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4232 * t10326 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t49889;
    t60479
}
