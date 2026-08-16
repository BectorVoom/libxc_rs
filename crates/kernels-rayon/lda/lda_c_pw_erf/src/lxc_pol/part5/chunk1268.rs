//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1268/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1268(t331: f64, t7761: f64, t11: f64, t22759: f64, t503: f64, t1243: f64, t22764: f64, t22713: f64, t9836: f64, t1953: f64, t22717: f64, t3536: f64) -> (f64, f64, f64, f64, f64) {
    let t22792 = t331 * t7761;
    let t22795 = t11 * t503 * t22759;
    let t22798 = t11 * t1243 * t22764;
    let t22801 = t11 * t9836 * t22713;
    let t22804 = t1953 * t3536 * t22717;
    (t22792, t22795, t22798, t22801, t22804)
}
