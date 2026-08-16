//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1072/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1072(t10162: f64, t2187: f64, t519: f64, t1472: f64, t5375: f64, t1381: f64, t1466: f64, t5320: f64, t571: f64, t2153: f64, t3742: f64, t1318: f64, t2151: f64, t549: f64, t575: f64) -> (f64, f64, f64, f64, f64) {
    let t12557 = t519 * t10162 * t2187;
    let t12558 = 8.0_f64 / 45.0_f64 * t12557;
    let t12560 = 4.0_f64 / 5.0_f64 * t1472 * t5375;
    let t12564 = 4.0_f64 / 5.0_f64 * t571 * t1466 * t5320 * t1381;
    let t12566 = 16.0_f64 / 15.0_f64 * t3742 * t2153;
    let t12570 = 16.0_f64 / 15.0_f64 * t1318 * t2151 * t575 * t549;
    (t12558, t12560, t12564, t12566, t12570)
}
