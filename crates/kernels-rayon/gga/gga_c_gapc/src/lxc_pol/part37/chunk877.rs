//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 877/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk877(t3727: f64, t787: f64, t2588: f64, t876: f64, t898: f64, t1033: f64, t7089: f64, t311: f64, t474: f64, t919: f64, t3288: f64, t7165: f64) -> (f64, f64, f64, f64) {
    let t9969 = t3727 * t787;
    let t9970 = t2588 * t9969;
    let t9972 = t3727 * t876;
    let t9973 = t898 * t9972;
    let t9975 = t7089 * t1033;
    let t9976 = t311 * t9975;
    let t9977 = t474 * t919;
    let t9978 = t9976 * t9977;
    let t9980 = t3288 * t7165;
    (t9970, t9973, t9978, t9980)
}
