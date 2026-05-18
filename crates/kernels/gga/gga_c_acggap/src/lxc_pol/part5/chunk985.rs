//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 985/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk985<F: Float>(t1476: F, t3143: F, t1049: F, t4833: F, t4823: F, t4819: F, t13698: F, t4811: F, t1072: F, t1298: F, t3124: F, t3126: F) -> (F, F, F, F, F, F) {
    let t16253 = t3143 * t1476;
    let t16255 = t1049 * t4833;
    let t16264 = t1049 * t4823;
    let t16274 = t1049 * t4819;
    let t16288 = t13698 * t4811;
    let t16292 = t3124 * t1072 * t1298 * t3126;
    (t16253, t16255, t16264, t16274, t16288, t16292)
}
