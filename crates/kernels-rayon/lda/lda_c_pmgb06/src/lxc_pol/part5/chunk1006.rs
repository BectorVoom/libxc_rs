//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1006/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1006(t350: f64, t365: f64, t7018: f64, t1271: f64, t2715: f64, t955: f64, t1238: f64, t2696: f64, t348: f64, t7015: f64, t2699: f64, t2707: f64, t410: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18721 = t365 * t7018 * t350;
    let t18725 = t1271 * t2715 * t955;
    let t18728 = t1238 * t2696 * t955;
    let t18731 = t348 * t7015 * t350;
    let t18734 = t1238 * t2699 * t955;
    let t18744 = t410 * t2707;
    (t18721, t18725, t18728, t18731, t18734, t18744)
}
