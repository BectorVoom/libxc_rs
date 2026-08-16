//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 921/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk921(t13580: f64, t4991: f64, t18089: f64, t695: f64, t2426: f64, t5005: f64, t5149: f64, t8232: f64, t4923: f64, t1636: f64, t5054: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t66578 = t13580 * t4991;
    let t66581 = t18089 * t695;
    let t66667 = t2426 * t5005;
    let t66720 = t8232 * t5149;
    let t66832 = t8232 * t4923;
    let t66833 = 8.0_f64 / 27.0_f64 * t66832;
    let t66902 = t89 * t1636 * t5054;
    (t66578, t66581, t66667, t66720, t66832, t66833, t66902)
}
