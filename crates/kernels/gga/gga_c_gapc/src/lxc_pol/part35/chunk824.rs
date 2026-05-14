//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 824/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk824<F: Float>(t11320: F, t611: F, t1720: F, t8950: F, t3137: F, t519: F) -> (F, F, F, F) {
    let t11321 = t611 * t11320;
    let t11322 = t1720 * t8950;
    let t11323 = t11321 * t11322;
    let t11325 = t519 * t3137;
    (t11321, t11322, t11323, t11325)
}
