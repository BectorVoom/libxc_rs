//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 819/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk819<F: Float>(t5005: F, t695: F, t1100: F, t52563: F, t13580: F, t4991: F, t18089: F, t2426: F, t5149: F, t8232: F, t4923: F, t1636: F, t5054: F, t89: F, t4935: F, t4926: F, t9733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t66520 = t695 * t5005;
    let t66555 = t1100 * t52563;
    let t66578 = t13580 * t4991;
    let t66581 = t18089 * t695;
    let t66667 = t2426 * t5005;
    let t66720 = t8232 * t5149;
    let t66832 = t8232 * t4923;
    let t66833 = 8.0 / 27.0 * t66832;
    let t66902 = t89 * t1636 * t5054;
    let t66903 = 4.0 / 9.0 * t66902;
    let t66905 = t89 * t1636 * t4935;
    let t66906 = 8.0 / 9.0 * t66905;
    let t66934 = t89 * t9733 * t4926;
    (t66520, t66555, t66578, t66581, t66667, t66720, t66832, t66833, t66902, t66903, t66905, t66906, t66934)
}
