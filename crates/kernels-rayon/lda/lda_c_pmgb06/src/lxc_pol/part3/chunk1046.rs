//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1046/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1046(t2945: f64, t831: f64, t1548: f64, t1887: f64, t2857: f64, t802: f64, t3134: f64, t815: f64, t1512: f64, t1874: f64, t161: f64, t3004: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12446 = t831 * t2945 / 30.0_f64;
    let t12447 = t1887 * t1548;
    let t12448 = t12447 / 45.0_f64;
    let t12449 = t802 * t2857;
    let t12450 = t12449 / 45.0_f64;
    let t12452 = t3134 * t815 / 30.0_f64;
    let t12454 = t1512 * t1874 / 10.0_f64;
    let t12456 = t161 * t3004 * t852;
    (t12446, t12448, t12450, t12452, t12454, t12456)
}
