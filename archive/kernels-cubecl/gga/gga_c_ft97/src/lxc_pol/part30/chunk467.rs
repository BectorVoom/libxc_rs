//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 467/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk467<F: Float>(t241: F, t258: F, t7536: F, t1449: F, t6154: F, t242: F) -> (F, F, F, F) {
    let t7538 = t241 * t7536 * t258;
    let t7542 = t6154 * t1449;
    let t7543 = t242 * t7542;
    let t7546 = t1449 * t1449;
    (t7538, t7542, t7543, t7546)
}
