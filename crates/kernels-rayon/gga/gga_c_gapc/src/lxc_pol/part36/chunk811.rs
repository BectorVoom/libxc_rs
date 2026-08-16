//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 811/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk811(t9695: f64, t9697: f64, t277: f64, t5463: f64, t2763: f64, t327: f64, t4043: f64, t7191: f64, t8754: f64, t5312: f64, t3708: f64, t7418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9698 = t9695 * t9697;
    let t9700 = t277 * t5463;
    let t9703 = t4043 * t327 * t2763 * t7191;
    let t9704 = t9700 * t9703;
    let t9706 = t277 * t8754;
    let t9707 = t9706 * t9703;
    let t9709 = t277 * t5312;
    let t9710 = t3708 * t7418;
    (t9698, t9703, t9704, t9707, t9709, t9710)
}
