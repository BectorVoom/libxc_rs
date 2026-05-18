//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 309/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk309<F: Float>(t3413: F, t515: F, t1053: F, t604: F, t379: F, t2210: F, t558: F, t920: F) -> (F, F, F, F) {
    let t3414 = t515 * t3413;
    let t3419 = t604 * t1053;
    let t3420 = t3419 * t379;
    let t3421 = t2210 * t3420;
    let t3424 = t920 * t558;
    (t3414, t3420, t3421, t3424)
}
