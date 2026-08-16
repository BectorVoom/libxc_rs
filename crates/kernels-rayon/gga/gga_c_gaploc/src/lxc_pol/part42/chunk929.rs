//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 929/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk929(t42378: f64, t11433: f64, t1415: f64, t7030: f64, t11426: f64, t9562: f64, t11318: f64, t1445: f64, t2293: f64, t574: f64, t13475: f64, t1580: f64) -> (f64, f64, f64, f64, f64) {
    let t46705 = 0.25561950635947166451e0_f64 * t42378;
    let t46707 = t1415 * t11433 * t7030;
    let t46708 = 0.14896037479937677779e-1_f64 * t46707;
    let t46709 = t11426 * t9562;
    let t46715 = 0.92023022289409799224e1_f64 * t574 * t1445 * t11318 * t2293;
    let t46717 = 0.43710935587469654631e2_f64 * t1580 * t13475;
    (t46705, t46708, t46709, t46715, t46717)
}
