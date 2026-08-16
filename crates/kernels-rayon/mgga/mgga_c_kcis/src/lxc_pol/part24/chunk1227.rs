//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1227/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1227(t28045: f64, t5078: f64, t1176: f64, t6681: f64, t1021: f64, t20191: f64, t19576: f64, t95474: f64, t19885: f64, t3227: f64, t5099: f64, t95381: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99999 = t28045 * t5078;
    let t100001 = t6681 * t1176;
    let t100003 = t1021 * t20191;
    let t100005 = t95474 * t19576;
    let t100007 = t3227 * t19885;
    let t100009 = t95381 * t5099;
    (t99999, t100001, t100003, t100005, t100007, t100009)
}
