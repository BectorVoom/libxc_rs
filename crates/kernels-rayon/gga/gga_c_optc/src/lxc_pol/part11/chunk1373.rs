//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1373/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1373(t43503: f64, t43508: f64, t44329: f64, t52446: f64, t52452: f64, t52591: f64, t52593: f64, t52596: f64, t52601: f64, t52687: f64, t52689: f64, t58435: f64) -> f64 {
    let t58448 = -0.19388333333333333333e1_f64 * t58435 + 0.14595555555555555556e-2_f64 * t52591 - 0.6568e-2_f64 * t52593 + 0.19704e-1_f64 * t52596 + 0.3284e-2_f64 * t52601 + 0.5170222222222222222e1_f64 * t52446 - 0.15510666666666666667e2_f64 * t52452 - 0.51702222222222222221e1_f64 * t43503 + 0.10340444444444444444e2_f64 * t43508 - 0.821e-2_f64 * t44329 + 0.3284e-2_f64 * t52687 - 0.19704e-1_f64 * t52689;
    t58448
}
