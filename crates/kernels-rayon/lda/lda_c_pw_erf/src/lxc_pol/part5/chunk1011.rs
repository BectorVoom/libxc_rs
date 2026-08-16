//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1011/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1011(t331: f64, t6812: f64, t325: f64, t6643: f64, t6640: f64, t1351: f64, t6005: f64, t6662: f64, t6659: f64, t1333: f64, t4606: f64, t6646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16287 = t331 * t6812;
    let t16292 = t325 * t6643;
    let t16297 = t325 * t6640;
    let t16305 = t1351 * t6005;
    let t16325 = t325 * t6662;
    let t16327 = t325 * t6659;
    let t16329 = t1333 * t6005;
    let t16338 = t4606 * t6646;
    (t16287, t16292, t16297, t16305, t16325, t16327, t16329, t16338)
}
