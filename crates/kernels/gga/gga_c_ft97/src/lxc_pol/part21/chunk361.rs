//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 361/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk361<F: Float>(t3188: F, t3440: F, t3439: F, t1017: F, t160: F, t379: F, t2221: F, t558: F) -> (F, F, F, F, F) {
    let t3441 = t3440 * t3188;
    let t3442 = t3439 * t3441;
    let t3445 = t160 * t1017;
    let t3446 = t3445 * t379;
    let t3447 = t2221 * t3446;
    let t3450 = t1017 * t558;
    (t3441, t3442, t3446, t3447, t3450)
}
