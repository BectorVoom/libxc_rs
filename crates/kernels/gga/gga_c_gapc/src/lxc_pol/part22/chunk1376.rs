//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1376/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1376<F: Float>(t33576: F, t33578: F, t33580: F, t33583: F, t33585: F, t33588: F, t33590: F, t33595: F, t33601: F, t33606: F, t36657: F, t33653: F) -> (F, F) {
    let t36658 = -F::cast_from(0.10793703140429833089e-5_f64) * t33576 - F::cast_from(0.12141398358188788626e-5_f64) * t33578 + F::cast_from(0.21587406280859666178e-5_f64) * t33580 + F::cast_from(0.10551281119038438161e-7_f64) * t33583 + F::cast_from(0.21102562238076876322e-7_f64) * t33585 - F::cast_from(0.22509399720615334744e-6_f64) * t33588 + F::cast_from(0.12817159869818982005e-5_f64) * t33590 + F::cast_from(0.20220636637604418766e-5_f64) * t33595 - F::cast_from(0.15018333275585850553e-5_f64) * t33601 - F::cast_from(0.20220636637604418766e-4_f64) * t33606 + t36657;
    let t36659 = F::cast_from(0.10298285674687440379e-4_f64) * t33653;
    (t36658, t36659)
}
