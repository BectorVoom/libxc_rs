//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 883/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk883(t3134: f64, t436: f64, t1512: f64, t1517: f64, t1447: f64, t3235: f64, t3243: f64, t3239: f64, t3251: f64, t1533: f64, t947: f64, t3109: f64, t350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9091 = t3134 * t436;
    let t9093 = t1512 * t1517;
    let t9104 = t1447 * t3235;
    let t9106 = t1447 * t3243;
    let t9108 = t1447 * t3239;
    let t9110 = t1447 * t3251;
    let t9147 = t947 * t1533;
    let t9149 = t350 * t3109;
    (t9091, t9093, t9104, t9106, t9108, t9110, t9147, t9149)
}
