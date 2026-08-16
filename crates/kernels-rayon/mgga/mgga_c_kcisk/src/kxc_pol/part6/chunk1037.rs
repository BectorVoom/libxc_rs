//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1037/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1037(t13185: f64, t30616: f64, t457: f64, t13233: f64, t30153: f64, t3530: f64, t1422: f64, t3533: f64, t2191: f64, t26572: f64, t3564: f64, t5932: f64, t7897: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31024 = t13185 * t30616;
    let t31025 = t457 * t31024;
    let t31034 = t3530 * t13233 * t30153;
    let t31038 = t1422 * t3533 * t30153;
    let t31041 = t26572 * t2191;
    let t31042 = t3564 * t31041;
    let t31045 = t5932 * t7897;
    (t31024, t31025, t31034, t31038, t31041, t31042, t31045)
}
