//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1074/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1074<F: Float>(t381: F, t4225: F, t879: F, t1648: F, t3243: F, t1160: F, t1539: F, t18906: F, t377: F, t5310: F, t1652: F, t980: F) -> (F, F, F, F, F) {
    let t19112 = t381 * t4225 * t879;
    let t19117 = t3243 * t1648;
    let t19122 = t1160 * t18906 * t1539;
    let t19129 = t377 * t5310;
    let t19133 = t980 * t1652;
    (t19112, t19117, t19122, t19129, t19133)
}
