//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1412/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1412<F: Float>(t30: F, t2257: F, t513: F, t9335: F, t9336: F, t9339: F, t9344: F, t33: F, t527: F, t1113: F, t3842: F, t3841: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t9348 = piecewise3::<F>(t31, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9335 * t9336 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t9339 * t2257 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t513 * t9344);
    let t9350 = F::cast_from(1.0_f64) / t527 / t33;
    let t9351 = t3842 * t1113;
    let t9354 = t3841 * t1113;
    let t9357 = -t9344;
    (t9348, t9350, t9351, t9354, t9357)
}
