//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2063/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2063<F: Float>(t21768: F, t38: F, t10389: F, t5819: F, t2299: F, t5825: F, t10398: F, t2306: F, t18281: F, t4186: F, t4227: F, t4232: F, t606: F, t633: F, t637: F) -> (F, F, F, F) {
    let t21769 = t38 * t21768;
    let t21784 = t10389 * t5819;
    let t21789 = t2299 * t5825;
    let t21794 = t10398 * t5819;
    let t21799 = t2306 * t5825;
    let t21804 = -F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t21784 * t606 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4227 * t4186 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t21789 * t606 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t633 * t18281 + F::cast_from(280.0_f64) / F::cast_from(27.0_f64) * t21794 * t606 + F::cast_from(56.0_f64) / F::cast_from(9.0_f64) * t4232 * t4186 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t21799 * t606 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t637 * t18281;
    (t21769, t21784, t21794, t21804)
}
