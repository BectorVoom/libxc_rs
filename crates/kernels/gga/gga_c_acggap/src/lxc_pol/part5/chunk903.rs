//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 903/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk903<F: Float>(t1005: F, t3531: F, t3348: F, t1165: F, t3451: F, t4210: F, t991: F, t1163: F, t955: F, t315: F, t4197: F, t1162: F) -> (F, F, F, F, F, F) {
    let t13539 = t1005 * t3531;
    let t13545 = t1005 * t3348;
    let t13573 = t3451 * t1165 * t991 * t4210;
    let t13582 = t1163 * t1165 * t991 * t955;
    let t13584 = t315 * t4197;
    let t13585 = t13584 * t1162;
    (t13539, t13545, t13573, t13582, t13584, t13585)
}
