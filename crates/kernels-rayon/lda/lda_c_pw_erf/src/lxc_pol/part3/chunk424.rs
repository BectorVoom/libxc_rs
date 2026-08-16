//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 424/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk424(t598: f64, t611: f64, t925: f64, t933: f64, t7: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1615 = t598 * t611;
    let t1619 = -0.55_f64 * t925 + 5.0_f64 / 18.0_f64 * t933;
    let t1620 = t1619 * pi;
    let t1621 = t1620 * t7;
    (t1615, t1619, t1620, t1621)
}
