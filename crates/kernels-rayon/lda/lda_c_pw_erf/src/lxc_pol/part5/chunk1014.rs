//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1014/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1014(t2146: f64, t4901: f64, t4763: f64, t4933: f64, t611: f64, t7280: f64, t1472: f64, t6685: f64, t1518: f64, t211: f64, t2527: f64, t2526: f64, t3975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16516 = t2146 * t4901;
    let t16520 = t4763 * t4933;
    let t16529 = t7280 * t611;
    let t16537 = t1472 * t6685;
    let t16600 = t211 * t1518 * t2527;
    let t16602 = t3975 * t2526;
    (t16516, t16520, t16529, t16537, t16600, t16602)
}
