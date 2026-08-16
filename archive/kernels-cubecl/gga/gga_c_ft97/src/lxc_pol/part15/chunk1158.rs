//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1158/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1158<F: Float>(t1526: F, t15567: F, t17687: F, t17694: F, t20489: F, t21181: F, t21196: F, t21204: F, t21408: F, t21453: F, t21464: F, t2320: F, t2321: F, t3806: F, t42279: F, t81968: F, t81971: F, t81974: F, t9490: F) -> F {
    let t89684 = -t1526 * t2320 * t2321 * t20489 / F::cast_from(12.0_f64) - t1526 * t2320 * t21453 / F::cast_from(4.0_f64) - t81968 / F::cast_from(12.0_f64) + t21464 + t81971 / F::cast_from(6.0_f64) - t81974 / F::cast_from(4.0_f64) - t1526 * t2320 * t9490 * t21181 / F::cast_from(2.0_f64) + t15567 * t17694 * t21204 / F::cast_from(2.0_f64) + t1526 * t2320 * t21408 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1526 * t3806 * t42279 * t21181 - t15567 * t17687 * t21196 / F::cast_from(3.0_f64);
    t89684
}
