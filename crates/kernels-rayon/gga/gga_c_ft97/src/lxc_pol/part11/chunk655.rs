//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 655/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk655(t2213: f64, t9099: f64, t2230: f64, t558: f64, t574: f64, t1882: f64, t2159: f64, t1647: f64, t569: f64, t616: f64, t2218: f64, t1554: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9100 = t9099 * t2213;
    let t9104 = t574 * t2230 * t558;
    let t9106 = t1882 * t2159;
    let t9109 = t569 * t616 * t1647;
    let t9112 = t1882 * t2218;
    let t9114 = t1554 * t525;
    (t9100, t9104, t9106, t9109, t9112, t9114)
}
