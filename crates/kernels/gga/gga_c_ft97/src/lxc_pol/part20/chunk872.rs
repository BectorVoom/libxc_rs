//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 872/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk872<F: Float>(t2354: F, t27787: F, t684: F, t6118: F, t24543: F, t6884: F, t24483: F, t24485: F, t24500: F, t24517: F, t24524: F, t27765: F, t27769: F, t27773: F, t27778: F, t27783: F) -> (F, F, F, F) {
    let t27789 = t2354 * t27787 * t684;
    let t27790 = t6118 * t27789;
    let t27792 = t24543 * t6884;
    let t27794 = t27765 / 9.0 - t27769 / 3.0 - t27773 / 12.0 - t27778 / 12.0 - t24483 + t24485 / 3.0 - t27783 - 2.0 / 3.0 * t24500 + t24517 / 6.0 - t24524 / 9.0 + t27790 / 6.0 - t27792 / 18.0;
    (t27789, t27790, t27792, t27794)
}
