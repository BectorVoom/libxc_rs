//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 547/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk547<F: Float>(t1008: F, t1200: F, t1195: F, t997: F, t336: F, t360: F) -> (F, F, F, F) {
    let t3271 = t1008 * t1200;
    let t3273 = t1008 * t1195;
    let t3280 = t997 * t1200;
    let t3282 = t336 * t360;
    (t3271, t3273, t3280, t3282)
}
