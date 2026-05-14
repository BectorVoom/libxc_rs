//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 498/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk498<F: Float>(t203: F, t328: F, t84: F, t281: F, t6: F) -> (F, F, F, F) {
    let t2604 = t203 * t328 * t84;
    let t2605 = t281 * t2604;
    let t2606 = 0.56968947174242584612e-3 * t2605;
    let t2607 = t6 * t328;
    (t2604, t2605, t2606, t2607)
}
