//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1056/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1056(t713: f64, t9596: f64, t2354: f64, t446: f64, t41490: f64, t724: f64, t2594: f64, t41473: f64, t2373: f64, t2409: f64, t9770: f64, t505: f64, t668: f64, t9692: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41930 = t9596 * t713;
    let t41932 = t446 * t2354 * t41930;
    let t41935 = t446 * t724 * t41490;
    let t41938 = t446 * t2594 * t41473;
    let t41940 = t2409 * t2373;
    let t41942 = t446 * t9770 * t41940;
    let t41945 = t9692 * t668 * t505;
    (t41930, t41932, t41935, t41938, t41940, t41942, t41945)
}
