//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1119/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1119(t10267: f64, t681: f64, t89: f64, t10270: f64, t2345: f64, t41448: f64, t10257: f64, t2336: f64, t2671: f64, t9733: f64, t10402: f64, t798: f64, t9568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43453 = t89 * t681 * t10267;
    let t43457 = t89 * t2345 * t10270 * t41448;
    let t43460 = t89 * t2336 * t10257;
    let t43463 = t89 * t9733 * t2671;
    let t43466 = t89 * t2336 * t10402;
    let t43468 = t9568 * t798;
    (t43453, t43457, t43460, t43463, t43466, t43468)
}
