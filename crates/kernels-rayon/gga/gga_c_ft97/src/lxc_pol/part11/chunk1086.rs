//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1086/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1086(t8392: f64, t9805: f64, t9810: f64, t10071: f64, t681: f64, t89: f64, t9976: f64, t2571: f64, t8232: f64, t2471: f64, t10067: f64, t1882: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42648 = t8392 * t9805;
    let t42650 = t8392 * t9810;
    let t42652 = t8392 * t10071;
    let t42690 = t89 * t681 * t9976;
    let t42697 = t8232 * t2571;
    let t42703 = t8232 * t2471;
    let t42708 = t1882 * t10067;
    (t42648, t42650, t42652, t42690, t42697, t42703, t42708)
}
