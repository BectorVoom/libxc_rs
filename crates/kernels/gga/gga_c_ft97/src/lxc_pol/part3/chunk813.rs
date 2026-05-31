//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 813/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk813<F: Float>(t3408: F, t72: F, t11280: F, t1526: F, t1527: F, t15567: F, t16631: F, t16634: F, t16641: F, t16644: F, t16649: F, t3313: F, t3323: F, t3338: F, t3414: F, t342: F, t343: F, t8759: F, t8761: F, t8764: F) -> F {
    let t16654 = t72 * t3408;
    let t16658 = t3313 + t3414 + t8759 - t8761 / F::cast_from(36.0_f64) - t8764 / F::cast_from(12.0_f64) - t16631 / F::cast_from(36.0_f64) - t15567 * t16634 / F::cast_from(9.0_f64) - t1526 * t1527 * t3323 / F::cast_from(12.0_f64) + t15567 * t16641 / F::cast_from(6.0_f64) - t1526 * t11280 * t16644 / F::cast_from(6.0_f64) - t16649 / F::cast_from(12.0_f64) - t1526 * t1527 * t3338 / F::cast_from(12.0_f64) - t342 * t343 * t16654 / F::cast_from(4.0_f64);
    t16658
}
