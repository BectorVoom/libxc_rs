//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1413/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1413<F: Float>(t33: F, t3351: F, t516: F, t9350: F, t9351: F, t9354: F, t9357: F, t162: F, t9348: F, zeta_threshold: F) -> F {
    let t34 = t33 <= zeta_threshold;
    let t9361 = piecewise3::<F>(t34, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9350 * t9351 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t9354 * t3351 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t516 * t9357);
    let t9363 = (t9348 + t9361) * t162;
    t9363
}
