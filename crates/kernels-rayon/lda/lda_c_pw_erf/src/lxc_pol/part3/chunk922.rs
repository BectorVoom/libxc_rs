//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 922/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk922(t1298: f64, t3550: f64, t1301: f64, t1518: f64, t493: f64, t2070: f64, t543: f64, t185: f64, t3553: f64, t511: f64, t1294: f64, t1498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9941 = t1298 * t3550;
    let t9944 = t493 * t1518 * t1301;
    let t9946 = t2070 * t543;
    let t9947 = t185 * t9946;
    let t9949 = t511 * t3553;
    let t9953 = t1498 * t1294;
    (t9941, t9944, t9946, t9947, t9949, t9953)
}
