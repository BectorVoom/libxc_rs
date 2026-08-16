//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 737/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk737(t1629: f64, t2106: f64, t137: f64, t132: f64, t1636: f64, t831: f64, t4838: f64, t4843: f64, t4846: f64, t4939: f64, t4943: f64, t4947: f64, t4950: f64, t4952: f64, t4956: f64, t4958: f64, t4960: f64, t4962: f64, t4964: f64) -> (f64, f64, f64, f64, f64) {
    let t4965 = t2106 * t1629;
    let t4966 = t137 * t4965;
    let t4968 = t132 * t4966 / 30.0_f64;
    let t4970 = 2.0_f64 / 45.0_f64 * t831 * t1636;
    let t4971 = t4838 - t4843 + t4846 - t4939 - t4943 - t4947 - t4950 - t4952 - t4956 - t4958 - t4960 - t4962 - t4964 - t4968 - t4970;
    (t4965, t4966, t4968, t4970, t4971)
}
