//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 483/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk483<F: Float>(t115: F, t2770: F, t426: F, t3209: F, t1724: F, t1168: F, t442: F, t1120: F) -> (F, F, F, F) {
    let t3211 = t426 * t2770 * t115;
    let t3212 = t3209 * t3211;
    let t3217 = t1724 * t3211;
    let t3233 = t1168 * t442;
    let t3234 = t3233 * t1120;
    (t3212, t3217, t3233, t3234)
}
