//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1037/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1037<F: Float>(t1349: F, t35027: F, t376: F, t1362: F, t148238: F, t148860: F, t148897: F, t148906: F, t148960: F, t149058: F, t149549: F, t149593: F, t2: F, t26: F, t26780: F, t27423: F, t27429: F, t32714: F, t35222: F, t4: F, t564: F, t7309: F) -> (F,) {
    let t149601 = t1349 * t376 * t35027;
    let t149607 = t32714 * t27423 / 9.0 - t32714 * t27429 / 27.0 - 2.0 * t148238 - t564 * t35222 - 2.0 * t149058 - 2.0 * t148860 - 2.0 * t148960 + (t149549 + t149593) * t2 * t4 * t26 * t1362 / 6.0 + t149601 / 9.0 + t7309 * t26780 / 6.0 - 4.0 * t148897 - 2.0 * t148906;
    (t149607,)
}
