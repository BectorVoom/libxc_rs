//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 936/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk936(t1393: f64, t1518: f64, t185: f64, t3546: f64, t514: f64, t4039: f64, t511: f64, t4036: f64, t568: f64, t1325: f64, t3787: f64, t3798: f64) -> (f64, f64, f64, f64, f64) {
    let t10422 = t185 * t1518 * t1393;
    let t10425 = t185 * t514 * t3546;
    let t10427 = t511 * t4039;
    let t10429 = t4036 * t568;
    let t10432 = t1325 * t3787 * t3798;
    (t10422, t10425, t10427, t10429, t10432)
}
