//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 908/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk908(t3807: f64, t519: f64, t9304: f64, t3762: f64, t581: f64, t1309: f64, t571: f64, t3828: f64, t3863: f64, t1325: f64, t3818: f64, t3859: f64) -> (f64, f64, f64, f64, f64) {
    let t9306 = t519 * t9304 * t3807;
    let t9313 = t3762 * t581;
    let t9315 = t571 * t9313 * t1309;
    let t9318 = t571 * t3863 * t3828;
    let t9338 = t1325 * t3859 * t3818;
    (t9306, t9313, t9315, t9318, t9338)
}
