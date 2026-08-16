//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1103/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1103(t1706: f64, t1711: f64, t1861: f64, t1878: f64, t20507: f64, t20513: f64, t20525: f64, t20529: f64, t2634: f64, t2642: f64, t3332: f64, t3339: f64, t444: f64, t450: f64, t7211: f64, t774: f64, t7957: f64, t7960: f64, t7974: f64, t9059: f64, t9068: f64) -> f64 {
    let t20533 = -3.0_f64 * t1861 * t7211 - 6.0_f64 * t9059 * t7957 + 24.0_f64 * t9068 * t7957 * t450 - 18.0_f64 * t3339 * t2634 * t1878 + 6.0_f64 * t3332 * t7960 - 18.0_f64 * t3339 * t7960 * t450 + 6.0_f64 * t1711 * t1878 * t2642 + 6.0_f64 * t1711 * t774 * t7211 - t1706 * t7974 + 2.0_f64 * t1711 * t7974 * t450 - t444 * (t20507 + t20513 + t20525 + t20529);
    t20533
}
