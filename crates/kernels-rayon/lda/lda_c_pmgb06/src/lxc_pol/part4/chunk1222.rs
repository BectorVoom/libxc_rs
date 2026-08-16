//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1222/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1222(t16106: f64, t1444: f64, t6131: f64, t16083: f64, t16087: f64, t16090: f64, t16092: f64, t16094: f64, t16095: f64, t16099: f64, t16100: f64, t16101: f64, t16102: f64, t16103: f64, t16105: f64) -> (f64, f64, f64) {
    let t16107 = 4.0_f64 / 405.0_f64 * t16106;
    let t16109 = 2.0_f64 / 45.0_f64 * t1444 * t6131;
    let t16110 = -t16083 - t16087 + t16090 - t16092 - t16094 - t16095 - t16099 - t16100 + t16101 - t16102 - t16103 - t16105 - t16107 + t16109;
    (t16107, t16109, t16110)
}
