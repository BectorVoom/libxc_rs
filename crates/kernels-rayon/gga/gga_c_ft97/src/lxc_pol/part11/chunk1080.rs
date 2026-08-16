//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1080/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1080(t761: f64, t9570: f64, t766: f64, t9571: f64, t1882: f64, t9989: f64, t10059: f64, t10004: f64, t2576: f64, t8232: f64, t241: f64, t41752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42416 = t761 * t9570;
    let t42417 = t9571 * t766;
    let t42422 = t1882 * t9989;
    let t42424 = t1882 * t10059;
    let t42430 = t1882 * t10004;
    let t42455 = t8232 * t2576;
    let t42469 = t41752 * t241;
    (t42416, t42417, t42422, t42424, t42430, t42455, t42469)
}
