//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 730/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk730(t332: f64, t6773: f64, t1385: f64, t439: f64, t1908: f64, t2002: f64, t4161: f64, t4162: f64, t4165: f64, t6733: f64, t6738: f64, t6740: f64, t6743: f64, t6746: f64, t6750: f64, t6754: f64, t6758: f64, t6763: f64, t6768: f64, t6772: f64) -> (f64, f64, f64, f64, f64) {
    let t6774 = t6773 * t332;
    let t6775 = t1385 * t6774;
    let t6777 = t439 * t6775 / 45.0_f64;
    let t6779 = 2.0_f64 / 45.0_f64 * t2002 * t1908;
    let t6780 = -t6733 - t6738 - t6740 - t4161 + 0.033245444444444446_f64 * t4162 + t4165 - t6743 - t6746 - t6750 + t6754 - t6758 - t6763 + t6768 - t6772 - t6777 - t6779;
    (t6774, t6775, t6777, t6779, t6780)
}
