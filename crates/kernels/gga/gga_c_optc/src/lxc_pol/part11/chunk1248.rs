//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1248/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1248<F: Float>(t104: F, t108: F, t13056: F, t176: F, t185: F, t1879: F, t20063: F, t203: F, t22434: F, t22439: F, t22657: F, t22659: F, t22661: F, t38936: F, t55933: F, t56062: F, t56068: F, t56638: F, t714: F, t95: F) -> F {
    let t56643 = t22434 - t22439 - t22657 + t56062 - t22659 - t22661 + t176 * t185 * t55933 * t108 * t203 / F::cast_from(2.0_f64) - t56068 + F::cast_from(140.0_f64) / F::cast_from(3.0_f64) * t38936 - F::cast_from(0.93041573165652349787e-1_f64) * t1879 * t13056 * t20063 + F::cast_from(0.25844881434903430496e-2_f64) * t95 * t104 * t56638 * t714;
    t56643
}
