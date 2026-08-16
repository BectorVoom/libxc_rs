//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 668/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk668(t1882: f64, t2174: f64, t2178: f64, t597: f64, t2180: f64, t144: f64, t2135: f64, t376: f64, t89: f64, t2157: f64, t558: f64, t574: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9274 = t1882 * t2174;
    let t9276 = t597 * t2178;
    let t9277 = t9276 * t2180;
    let t9278 = t144 * t9277;
    let t9282 = t89 * t376 * t2135;
    let t9284 = t2157 * t558;
    let t9286 = t574 * t605 * t9284;
    (t9274, t9276, t9277, t9278, t9282, t9284, t9286)
}
