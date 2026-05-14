//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 582/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk582<F: Float>(t11119: F, t11120: F, t3056: F, t77: F, t3020: F, t1771: F, t926: F, t3044: F, t458: F, t3047: F, t14: F, t7741: F, t12: F, t9: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11121 = t11119 * t11120;
    let t11135 = t77 * t3056;
    let t11136 = t3020 * t11135;
    let t11167 = t1771 * t926;
    let t11169 = t458 * t3044;
    let t11170 = 4.0 / 27.0 * t11169;
    let t11171 = t458 * t3047;
    let t11172 = 4.0 / 9.0 * t11171;
    let t11174 = 1.0 / t14 / t7741;
    let t11175 = t12 * t11174;
    let t11176 = t9 * t11175;
    (t11121, t11136, t11167, t11169, t11170, t11171, t11172, t11174, t11175, t11176)
}
