//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1138/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1138(t3408: f64, t4738: f64, t2171: f64, t3812: f64, t3413: f64, t1325: f64, t5289: f64, t542: f64, t806: f64, t944: f64, t1278: f64, t5290: f64) -> (f64, f64, f64, f64, f64) {
    let t13325 = 16.0_f64 / 15.0_f64 * t4738 * t3408;
    let t13327 = 8.0_f64 / 15.0_f64 * t2171 * t3812;
    let t13329 = 8.0_f64 / 15.0_f64 * t2171 * t3413;
    let t13334 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t806 * t944 * t542;
    let t13338 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t5290 * t1278;
    (t13325, t13327, t13329, t13334, t13338)
}
