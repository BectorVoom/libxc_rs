//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 945/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk945(t1631: f64, t4183: f64, t1634: f64, t474: f64, t602: f64, t1210: f64, t1638: f64, t603: f64, t1639: f64, t20: f64, t3945: f64, t4196: f64) -> (f64, f64, f64, f64, f64) {
    let t10690 = t1631 * t4183;
    let t10694 = 0.38474813732852775_f64 * t602 * t474 * t1634;
    let t10697 = 0.019878653761973935_f64 * t1638 * t1210 * t603;
    let t10699 = t3945 * t20 * t1639;
    let t10702 = t1631 * t4196;
    (t10690, t10694, t10697, t10699, t10702)
}
