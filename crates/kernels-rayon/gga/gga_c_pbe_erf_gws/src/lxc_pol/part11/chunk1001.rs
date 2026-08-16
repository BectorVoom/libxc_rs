//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1001/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1001(t3703: f64, t3799: f64, t6480: f64, t1114: f64, t346: f64, t38375: f64, t3863: f64, t6717: f64, t3916: f64, t6566: f64, t3867: f64, t21293: f64, t3841: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39052 = t3703 * param_a_c;
    let t39082 = t6480 * t3799;
    let t39095 = t1114 * t38375 * t346;
    let t39174 = t6717 * t3863;
    let t39181 = t3916 * t6566;
    let t39191 = t6480 * t3867;
    let t39388 = t21293 * t3841;
    (t39052, t39082, t39095, t39174, t39181, t39191, t39388)
}
