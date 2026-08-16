//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1024(t1325: f64, t1326: f64, t2005: f64, t2961: f64, t4804: f64, t5266: f64, t3794: f64, t2954: f64, t3518: f64, t5250: f64, t784: f64, t3838: f64, t4763: f64) -> (f64, f64, f64, f64, f64) {
    let t11999 = 8.0_f64 / 45.0_f64 * t1325 * t1326 * t2005 * t2961;
    let t12001 = 8.0_f64 / 9.0_f64 * t4804 * t5266;
    let t12003 = 8.0_f64 / 9.0_f64 * t3794 * t5266;
    let t12008 = 64.0_f64 / 81.0_f64 * t1325 * t5250 * t784 * t3518 * t2954;
    let t12010 = 16.0_f64 / 15.0_f64 * t4763 * t3838;
    (t11999, t12001, t12003, t12008, t12010)
}
