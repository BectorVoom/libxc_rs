//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 820/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk820(t2093: f64, t2623: f64, t166: f64, t161: f64, t2625: f64, t831: f64, t2592: f64, t824: f64, t2631: f64, t802: f64, t6611: f64, t6614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7747 = t2093 * t2623;
    let t7748 = t166 * t7747;
    let t7750 = t161 * t7748 / 10.0_f64;
    let t7752 = t831 * t2625 / 10.0_f64;
    let t7754 = t2592 * t824 / 10.0_f64;
    let t7756 = t802 * t2631 / 5.0_f64;
    let t7758 = 2.0_f64 / 15.0_f64 * t6611;
    let t7759 = 2.0_f64 / 15.0_f64 * t6614;
    (t7747, t7748, t7750, t7752, t7754, t7756, t7758, t7759)
}
