//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1112/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1112(t10406: f64, t1882: f64, t2413: f64, t2682: f64, t10248: f64, t446: f64, t824: f64, t9587: f64, t2665: f64, t41482: f64, t835: f64, t2857: f64, t41464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43365 = t1882 * t10406;
    let t43367 = t2413 * t2682;
    let t43369 = t446 * t10248 * t43367;
    let t43371 = t9587 * t824;
    let t43373 = t446 * t2665 * t43371;
    let t43376 = t446 * t835 * t41482;
    let t43379 = t446 * t2857 * t41464;
    (t43365, t43367, t43369, t43371, t43373, t43376, t43379)
}
