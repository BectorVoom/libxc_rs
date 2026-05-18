//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1377/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1377<F: Float>(t33576: F, t33578: F, t33580: F, t33583: F, t33585: F, t33588: F, t33590: F, t33595: F, t33601: F, t33606: F, t36657: F, t33653: F) -> (F, F) {
    let t36658 = -F::new(0.10793703140429833089e-5) * t33576 - F::new(0.12141398358188788626e-5) * t33578 + F::new(0.21587406280859666178e-5) * t33580 + F::new(0.10551281119038438161e-7) * t33583 + F::new(0.21102562238076876322e-7) * t33585 - F::new(0.22509399720615334744e-6) * t33588 + F::new(0.12817159869818982005e-5) * t33590 + F::new(0.20220636637604418766e-5) * t33595 - F::new(0.15018333275585850553e-5) * t33601 - F::new(0.20220636637604418766e-4) * t33606 + t36657;
    let t36659 = F::new(0.10298285674687440379e-4) * t33653;
    (t36658, t36659)
}
