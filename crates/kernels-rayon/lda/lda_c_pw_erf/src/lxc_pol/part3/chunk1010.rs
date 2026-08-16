//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1010/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1010(t2061: f64, t803: f64, t1953: f64, t3411: f64, t503: f64, t790: f64, t1243: f64, t3481: f64, t10967: f64, t21: f64, t2095: f64, t1977: f64, t8930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11829 = t2061 * t803;
    let t11832 = t1953 * t503 * t3411;
    let t11834 = t1953 * t790;
    let t11837 = t1953 * t1243 * t3481;
    let t11845 = t21 * t10967;
    let t11846 = t11845 * t2095;
    let t11848 = t8930 * t1977;
    (t11829, t11832, t11834, t11837, t11845, t11846, t11848)
}
