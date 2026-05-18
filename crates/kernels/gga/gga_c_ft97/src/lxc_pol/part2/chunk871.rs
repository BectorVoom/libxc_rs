//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 871/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk871<F: Float>(t13571: F, t200: F, t2379: F, t2382: F, t2417: F, t222: F, t226: F, t1113: F, t236: F, t1127: F, t694: F, t3724: F) -> (F, F, F, F) {
    let t13572 = t13571 * t200;
    let t13577 = t2379 * t2417 * t2382;
    let t13580 = t222 * t226;
    let t13581 = t236 * t1113;
    let t13582 = t13580 * t13581;
    let t13585 = t694 * t1127;
    let t13586 = t3724 * t13585;
    (t13572, t13577, t13582, t13586)
}
