//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 672/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk672(t143: f64, t7954: f64, t167: f64, t7955: f64, t1651: f64, t569: f64, t616: f64, t1643: f64, t2205: f64, t1882: f64, t2144: f64, t2170: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9327 = t7954 * t143;
    let t9329 = t9327 * t167 * t7955;
    let t9333 = t569 * t616 * t1651;
    let t9337 = t2205 * t616 * t1643;
    let t9340 = t1882 * t2144;
    let t9342 = t1882 * t2170;
    (t9327, t9329, t9333, t9337, t9340, t9342)
}
