//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 967/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk967<F: Float>(t21253: F, t25: F, t1127: F, t5001: F, t80002: F, t9524: F, t1100: F, t66520: F, t1882: F, t21641: F, t21488: F, t21639: F, t761: F) -> (F, F, F, F, F, F, F) {
    let t80128 = t21253 * t25;
    let t80149 = t5001 * t1127;
    let t80157 = t9524 * t80002;
    let t80167 = t1100 * t66520;
    let t80212 = t1882 * t21641;
    let t80316 = t1882 * t21488;
    let t80334 = t761 * t21639;
    (t80128, t80149, t80157, t80167, t80212, t80316, t80334)
}
