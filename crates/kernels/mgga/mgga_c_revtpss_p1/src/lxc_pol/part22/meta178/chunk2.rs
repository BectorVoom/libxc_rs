//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1167/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1167<F: Float>(t1471: F, t1487: F, t1494: F, t4182: F, t4188: F, t4191: F, t4196: F, t4218: F, t4238: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> F {
    let t4241 = -t4182 * t85 / F::cast_from(12.0_f64) - t4188 * t85 / F::cast_from(12.0_f64) - t4191 * t85 / F::cast_from(12.0_f64) - t1471 * t641 / F::cast_from(12.0_f64) - t4196 * t85 / F::cast_from(12.0_f64) + t4218 * t85 / F::cast_from(24.0_f64) + t1487 * t641 / F::cast_from(24.0_f64) - t608 * t1494 / F::cast_from(12.0_f64) + t628 * t1494 / F::cast_from(24.0_f64) + t71 * t4238 / F::cast_from(24.0_f64);
    t4241
}
