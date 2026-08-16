//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 372/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk372(t1333: f64, t219: f64, t951: f64, t574: f64, t571: f64, t1251: f64, t197: f64, t940: f64, t522: f64, t519: f64, t1237: f64, t174: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1334 = t219 * t1333;
    let t1335 = t1334 * t951;
    let t1336 = t574 * t1335;
    let t1338 = 8.0_f64 / 45.0_f64 * t571 * t1336;
    let t1339 = t197 * t1251;
    let t1340 = t1339 * t940;
    let t1341 = t522 * t1340;
    let t1343 = 8.0_f64 / 45.0_f64 * t519 * t1341;
    let t1345 = t174 * t1237 * t205;
    (t1334, t1335, t1336, t1338, t1339, t1340, t1341, t1343, t1345)
}
