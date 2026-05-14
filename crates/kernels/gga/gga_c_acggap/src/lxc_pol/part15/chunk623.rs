//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 623/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk623<F: Float>(t2096: F, t7310: F, t2015: F, t2028: F, t2048: F, t2016: F, t2052: F, t594: F, t8: F, t130: F) -> (F, F, F, F, F) {
    let t7311 = t7310 * t2096;
    let t7315 = t2015 * t2028;
    let t7316 = t7315 * t2048;
    let t7317 = 11.0 / 192.0 * t7316;
    let t7318 = t2016 * t2052;
    let t7319 = 11.0 / 576.0 * t7318;
    let t7321 = 1.0 / t8 / t594;
    let t7322 = t130 * t7321;
    (t7311, t7315, t7317, t7319, t7322)
}
