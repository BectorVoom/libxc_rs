//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1016/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1016(t2497: f64, t4489: f64, t1325: f64, t3859: f64, t6264: f64, t2171: f64, t5421: f64, t3863: f64, t571: f64, t6408: f64, t3854: f64, t6413: f64) -> (f64, f64, f64, f64, f64) {
    let t16657 = t4489 * t2497;
    let t16702 = t1325 * t3859 * t6264;
    let t16709 = t2171 * t5421;
    let t16762 = t571 * t3863 * t6408;
    let t16765 = t571 * t3854 * t6413;
    (t16657, t16702, t16709, t16762, t16765)
}
