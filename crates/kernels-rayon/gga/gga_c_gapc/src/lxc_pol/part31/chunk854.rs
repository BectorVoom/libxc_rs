//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 854/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk854(t2763: f64, t327: f64, t4043: f64, t7191: f64, t9700: f64, t277: f64, t8754: f64, t5312: f64, t3708: f64, t7418: f64, t8986: f64, t961: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9703 = t4043 * t327 * t2763 * t7191;
    let t9704 = t9700 * t9703;
    let t9706 = t277 * t8754;
    let t9707 = t9706 * t9703;
    let t9709 = t277 * t5312;
    let t9710 = t3708 * t7418;
    let t9711 = t9709 * t9710;
    let t9713 = t8986 * t961;
    (t9703, t9704, t9707, t9709, t9711, t9713)
}
