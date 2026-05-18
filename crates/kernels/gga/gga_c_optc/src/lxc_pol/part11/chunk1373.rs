//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1373/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1373<F: Float>(t43503: F, t43508: F, t44329: F, t52446: F, t52452: F, t52591: F, t52593: F, t52596: F, t52601: F, t52687: F, t52689: F, t58435: F) -> F {
    let t58448 = -F::new(0.19388333333333333333e1) * t58435 + F::new(0.14595555555555555556e-2) * t52591 - F::new(0.6568e-2) * t52593 + F::new(0.19704e-1) * t52596 + F::new(0.3284e-2) * t52601 + F::new(0.5170222222222222222e1) * t52446 - F::new(0.15510666666666666667e2) * t52452 - F::new(0.51702222222222222221e1) * t43503 + F::new(0.10340444444444444444e2) * t43508 - F::new(0.821e-2) * t44329 + F::new(0.3284e-2) * t52687 - F::new(0.19704e-1) * t52689;
    t58448
}
