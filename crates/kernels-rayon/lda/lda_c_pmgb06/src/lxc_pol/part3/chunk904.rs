//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 904/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk904(t1423: f64, t3217: f64, t1447: f64, t3195: f64, t1427: f64, t3220: f64, t1511: f64, t607: f64, t446: f64, t3012: f64, t3204: f64, t161: f64, t3446: f64, t489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9830 = t1423 * t3217;
    let t9832 = t1447 * t3195;
    let t9834 = t3220 * t1427;
    let t9836 = t1511 * t607;
    let t9837 = t9836 * t446;
    let t9847 = t1423 * t3012;
    let t9853 = t1447 * t3204;
    let t9885 = t161 * t489 * t3446;
    (t9830, t9832, t9834, t9836, t9837, t9847, t9853, t9885)
}
