//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1351/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1351(t10319: f64, t10321: f64, t10335: f64, t10339: f64, t161: f64, t489: f64, t6832: f64, t10743: f64, t10746: f64, t10751: f64, t10754: f64, t10757: f64, t10760: f64, t10764: f64, t10769: f64, t10770: f64, t10773: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17766 = 4.0_f64 / 405.0_f64 * t10319;
    let t17767 = 16.0_f64 / 1215.0_f64 * t10321;
    let t17768 = 16.0_f64 / 1215.0_f64 * t10335;
    let t17769 = 4.0_f64 / 405.0_f64 * t10339;
    let t17771 = t161 * t489 * t6832;
    let t17772 = 2.0_f64 / 45.0_f64 * t17771;
    let t17779 = -t17766 + t17767 + t17768 + t17769 + t17772 + 0.4328416544945937_f64 * t10743 + t10746 + 0.21642082724729686_f64 * t10751 + 0.011181742741110338_f64 * t10754 + 0.6492624817418906_f64 * t10757 + 0.06709045644666203_f64 * t10760 + t10764 + t10769 - 0.19237406866426388_f64 * t10770 - t10773;
    (t17766, t17767, t17768, t17769, t17772, t17779)
}
