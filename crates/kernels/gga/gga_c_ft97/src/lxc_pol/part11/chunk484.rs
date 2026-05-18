//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 484/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk484<F: Float>(t2725: F, t2726: F, t2434: F, t2437: F, t2444: F, t2449: F, t2453: F) -> (F, F) {
    let t2727 = t2725 * t2726;
    let t2730 = F::new(0.11113000182098765433e-1) * t2434;
    let t2735 = -t2730 + F::new(0.11113000182098765433e-1) * t2437 + F::new(0.22226000364197530865e-1) * t2444 - F::new(0.33339000546296296298e-1) * t2449 + F::new(0.16669500273148148149e-1) * t2453;
    (t2727, t2735)
}
