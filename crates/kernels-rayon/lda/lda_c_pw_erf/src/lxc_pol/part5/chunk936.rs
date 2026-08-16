//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 936/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk936(t153: f64, t2869: f64, t678: f64, t1159: f64, t632: f64, t1143: f64, t695: f64, t2929: f64, t466: f64, t10953: f64, t148: f64, t1198: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11204 = t153 * t2869 * t678;
    let t11219 = t1159 * t632;
    let t11222 = 1.0051538464260528_f64 * t695 * t1143;
    let t11229 = t466 * t2929;
    let t11232 = 0.0837628205355044_f64 * t148 * t10953;
    let t11233 = t1198 * t1143;
    (t11204, t11219, t11222, t11229, t11232, t11233)
}
