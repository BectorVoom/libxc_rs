//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 952/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk952(t1969: f64, t39735: f64, t446: f64, t558: f64, t7973: f64, t37264: f64, t569: f64, t2205: f64, t37269: f64, t378: f64, t7368: f64, t358: f64, t363: f64, t9017: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39737 = t446 * t1969 * t39735;
    let t39739 = t7973 * t558;
    let t39741 = t446 * t1969 * t39739;
    let t39744 = t446 * t569 * t37264;
    let t39747 = t446 * t2205 * t37269;
    let t39749 = t378 * t7368;
    let t39751 = t9017 * t358 * t363;
    (t39737, t39739, t39741, t39744, t39747, t39749, t39751)
}
