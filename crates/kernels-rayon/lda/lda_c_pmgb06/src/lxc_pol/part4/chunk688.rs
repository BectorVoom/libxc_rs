//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 688/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk688(t1147: f64, t123: f64, t317: f64, t701: f64, t1126: f64, t740: f64, t1321: f64, t67: f64, t374: f64, t73: f64, t1289: f64, t107: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4030 = t123 * t1147 * t701 * t317;
    let t4034 = t123 * t740 * t1126 * t317;
    let t4042 = 1.0_f64 / t1321 / t67;
    let t4044 = t374 * t374;
    let t4045 = t73 * t4044;
    let t4053 = t73 * t1289;
    let t4060 = t107 * t410 * t1126;
    (t4030, t4034, t4042, t4044, t4045, t4053, t4060)
}
