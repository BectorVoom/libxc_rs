//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1043/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1043<F: Float>(t26467: F, t26470: F, t26431: F, t26434: F, t26437: F, t26441: F, t26444: F, t26448: F, t26451: F, t26454: F, t26457: F, t26460: F, t26468: F) -> F {
    let t26471 = t26470 * t26467;
    let t26473 = -F::new(0.5405960648148148148e-2) * t26431 + F::new(0.18571777777777777777e-1) * t26434 + F::new(0.69644166666666666665e-2) * t26437 + F::new(0.13928833333333333333e-1) * t26441 - F::new(0.13928833333333333333e-1) * t26444 - F::new(0.69644166666666666665e-2) * t26448 + F::new(0.32435763888888888888e-2) * t26451 - F::new(0.18571777777777777777e-1) * t26454 + F::new(0.21667074074074074073e-1) * t26457 - F::new(0.69505208333333333333e-3) * t26460 - F::new(0.13901041666666666667e-2) * t26468 - F::new(0.18550940104166666667e-3) * t26471;
    t26473
}
