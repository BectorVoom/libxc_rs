//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1036/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1036(t188: f64, t3023: f64, t398: f64, t4641: f64, t4913: f64, t83: f64, t1166: f64, t1409: f64, t1366: f64, t3315: f64, t3322: f64, t27: f64, t3018: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10720 = t398 * t3023 * t188;
    let t10727 = 4.0_f64 / 3.0_f64 * t83 * (-4.277777777777778_f64 * t4641 + 220.0_f64 / 81.0_f64 * t4913) * t188;
    let t10735 = t1166 * t1409 * t188;
    let t10743 = t3315 * t1366;
    let t10746 = 0.4328416544945937_f64 * t3322 * t1366;
    let t10751 = t3018 * t27 * t545;
    (t10720, t10727, t10735, t10743, t10746, t10751)
}
