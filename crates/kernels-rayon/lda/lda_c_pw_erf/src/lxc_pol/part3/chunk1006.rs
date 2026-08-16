//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1006/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1006(t4606: f64, t4634: f64, t11766: f64, t1953: f64, t503: f64, t325: f64, t4629: f64, t2954: f64, t4614: f64, t11: f64, t2092: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11770 = t4606 * t4634;
    let t11773 = t1953 * t503 * t11766;
    let t11775 = t325 * t4629;
    let t11777 = t4614 * t2954;
    let t11779 = t11 * t503 * t11777;
    let t11781 = t933 * t2092;
    (t11770, t11773, t11775, t11777, t11779, t11781)
}
