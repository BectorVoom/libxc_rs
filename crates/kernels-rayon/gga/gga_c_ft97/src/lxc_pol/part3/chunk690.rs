//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 690/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk690(t1882: f64, t3277: f64, t3273: f64, t3268: f64, t10992: f64, t11021: f64, t11023: f64, t11025: f64, t11043: f64, t3155: f64, t458: f64, t1771: f64, t963: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11610 = 2.0_f64 / 27.0_f64 * t1882 * t3277;
    let t11612 = 2.0_f64 / 9.0_f64 * t1882 * t3273;
    let t11632 = 4.0_f64 / 9.0_f64 * t1882 * t3268;
    let t11638 = 2.0_f64 / 27.0_f64 * t10992;
    let t11646 = 2.0_f64 / 27.0_f64 * t11021;
    let t11647 = 4.0_f64 / 27.0_f64 * t11023;
    let t11648 = 4.0_f64 / 81.0_f64 * t11025;
    let t11659 = 4.0_f64 / 81.0_f64 * t11043;
    let t11668 = 2.0_f64 / 3.0_f64 * t458 * t3155;
    let t11669 = t1771 * t963;
    (t11610, t11612, t11632, t11638, t11646, t11647, t11648, t11659, t11668, t11669)
}
