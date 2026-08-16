//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 987/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk987(t4913: f64, t6888: f64, t4641: f64, t6816: f64, t350: f64, t6805: f64, t6824: f64, t6821: f64, t6802: f64, t6808: f64, t6813: f64, t405: f64, t6882: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17129 = t4913 * t6888;
    let t17131 = t4641 * t6816;
    let t17133 = t350 * t6805;
    let t17138 = t350 * t6824;
    let t17140 = t350 * t6821;
    let t17164 = t350 * t6802;
    let t17166 = t4641 * t6808;
    let t17177 = t350 * t6813;
    let t17185 = t405 * t6882;
    (t17129, t17131, t17133, t17138, t17140, t17164, t17166, t17177, t17185)
}
