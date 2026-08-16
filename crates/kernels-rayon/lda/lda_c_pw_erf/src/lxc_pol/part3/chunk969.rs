//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 969/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk969(t242: f64, t4120: f64, t1159: f64, t632: f64, t1143: f64, t695: f64, t9196: f64, t4100: f64, t1203: f64, t2929: f64, t466: f64, t10953: f64, t148: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11217 = t4120 * t242;
    let t11219 = t1159 * t632;
    let t11222 = 1.0051538464260528_f64 * t695 * t1143;
    let t11223 = t9196 * t242;
    let t11225 = t4100 * t632;
    let t11227 = t1203 * t1143;
    let t11229 = t466 * t2929;
    let t11232 = 0.0837628205355044_f64 * t148 * t10953;
    (t11217, t11219, t11222, t11223, t11225, t11227, t11229, t11232)
}
