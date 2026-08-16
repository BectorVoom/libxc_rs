//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1320/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1320(t11213: f64, t2295: f64, t11319: f64, t11322: f64, t11326: f64, t22503: f64, t2328: f64, t237: f64, t31058: f64, t31472: f64, t31521: f64, t31523: f64, t31582: f64, t31584: f64, t31586: f64, t3161: f64, t891: f64, t898: f64, t9985: f64) -> f64 {
    let t31969 = t2295 * t11213;
    let t31986 = 0.11696447245269292414e1_f64 * t898 * t31969 * t891 + 0.19751673498613801407e-1_f64 * t237 * t31472 + 0.31168546390226634766e3_f64 * t22503 * t9985 * t31058 - t31521 + t31523 + t31582 + t31584 + t31586 - 0.6233709278045326953e3_f64 * t898 * t11322 * t3161 + 0.14035736694323150897e2_f64 * t898 * t11326 * t891 - 0.51947577317044391277e2_f64 * t2328 * t11319;
    t31986
}
