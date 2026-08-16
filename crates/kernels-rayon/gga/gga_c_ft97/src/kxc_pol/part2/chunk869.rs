//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 869/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk869(t13352: f64, t2404: f64, t92: f64, t13320: f64, t3051: f64, t13309: f64, t13346: f64, t683: f64, t13301: f64, t13296: f64, t665: f64, t668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13549 = t2404 * t13352;
    let t13550 = t92 * t13549;
    let t13552 = t2404 * t13320;
    let t13553 = t3051 * t13552;
    let t13555 = t2404 * t13309;
    let t13556 = t92 * t13555;
    let t13558 = t683 * t13346;
    let t13559 = t92 * t13558;
    let t13561 = t683 * t13301;
    let t13562 = t3051 * t13561;
    let t13564 = t683 * t13296;
    let t13565 = t92 * t13564;
    let t13567 = t665 * t668;
    (t13550, t13553, t13556, t13559, t13562, t13565, t13567)
}
