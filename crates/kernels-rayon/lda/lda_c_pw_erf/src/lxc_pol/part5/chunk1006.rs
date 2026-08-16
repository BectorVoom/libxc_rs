//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1006/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1006(t519: f64, t6427: f64, t9304: f64, t13432: f64, t6464: f64, t1325: f64, t3859: f64, t6468: f64, t2388: f64, t571: f64, t9313: f64, t1518: f64, t185: f64, t2472: f64) -> (f64, f64, f64, f64, f64) {
    let t16042 = t519 * t9304 * t6427;
    let t16050 = t519 * t13432 * t6464;
    let t16053 = t1325 * t3859 * t6468;
    let t16058 = t571 * t9313 * t2388;
    let t16065 = t185 * t1518 * t2472;
    (t16042, t16050, t16053, t16058, t16065)
}
