//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 583/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk583<F: Float>(t1077: F, t322: F, t368: F, t398: F, t384: F, t377: F, t951: F) -> (F, F, F, F) {
    let t3730 = t1077 * t322;
    let t3732 = t398 * t368 * t3730;
    let t3733 = t384 * t3732;
    let t3740 = t377 * t951;
    (t3730, t3732, t3733, t3740)
}
