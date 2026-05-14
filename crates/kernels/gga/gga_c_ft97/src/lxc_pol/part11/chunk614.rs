//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 614/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk614<F: Float>(t1546: F, t1975: F, t89: F, t1636: F, t559: F, t2076: F, t375: F, t10: F, t144: F, t3050: F, t1984: F, t378: F, t1986: F, t379: F, t446: F, t1647: F, t558: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9062 = t89 * t1546 * t1975;
    let t9065 = t89 * t1636 * t559;
    let t9068 = t89 * t375 * t2076;
    let t9071 = t10 * t3050 * t144;
    let t9072 = 14.0 / 81.0 * t9071;
    let t9073 = t378 * t1984;
    let t9074 = t379 * t1986;
    let t9075 = t9073 * t9074;
    let t9076 = t446 * t9075;
    let t9078 = t1647 * t558;
    (t9062, t9065, t9068, t9071, t9072, t9073, t9074, t9075, t9076, t9078)
}
