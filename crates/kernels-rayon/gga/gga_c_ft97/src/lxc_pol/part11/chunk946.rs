//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 946/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk946(t39660: f64, t446: f64, t9049: f64, t7788: f64, t9348: f64, t1969: f64, t379: f64, t9293: f64, t9073: f64, t10: f64, t11175: f64, t144: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39662 = t446 * t9049 * t39660;
    let t39664 = t7788 * t9348;
    let t39666 = t446 * t1969 * t39664;
    let t39668 = t9293 * t379;
    let t39670 = t446 * t9073 * t39668;
    let t39673 = t10 * t11175 * t144;
    (t39662, t39664, t39666, t39668, t39670, t39673)
}
