//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 886/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk886<F: Float>(t3765: F, t957: F, t1036: F, t1426: F, t175: F, t864: F, t922: F, t1032: F, t3493: F, t1092: F, t3228: F, t3357: F, t3775: F) -> (F, F, F, F, F) {
    let t13100 = t3765 * t957;
    let t13110 = t1036 * t1426 * t175 * t922 * t864;
    let t13112 = t1032 * t3493;
    let t13121 = t3228 * t1092;
    let t13128 = t3775 * t3357;
    (t13100, t13110, t13112, t13121, t13128)
}
