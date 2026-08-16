//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1008/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1008(t331: f64, t5084: f64, t5087: f64, t5097: f64, t5100: f64, t2954: f64, t739: f64, t9777: f64, t11: f64, t3536: f64, t174: f64, t4641: f64, t9810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11793 = t331 * t5084;
    let t11798 = t331 * t5087;
    let t11803 = t331 * t5097;
    let t11805 = t331 * t5100;
    let t11808 = t9777 * t739 * t2954;
    let t11813 = t11 * t3536 * t11808;
    let t11818 = t174 * t9810 * t4641;
    (t11793, t11798, t11803, t11805, t11808, t11813, t11818)
}
