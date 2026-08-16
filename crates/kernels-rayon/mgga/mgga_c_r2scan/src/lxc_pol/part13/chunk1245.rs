//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1245/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1245(t322: f64, t12005: f64, t1338: f64, t40851: f64, t3678: f64, t6755: f64, t1348: f64, t6767: f64, t11145: f64, t11157: f64, t12002: f64, t12009: f64, t2438: f64, t31929: f64, t3413: f64, t352: f64, t3675: f64, t37199: f64, t37218: f64, t6751: f64, t8481: f64, t855: f64, t9760: f64) -> f64 {
    let t332 = 0.25e1_f64 < t322;
    let t41028 = t1338 * t12005;
    let t41033 = piecewise3(t332, t40851, 0.0_f64);
    let t41039 = t6755 * t3678;
    let t41042 = t1348 * t12005;
    let t41047 = t6767 * t3678;
    let t41054 = -0.21e1_f64 * t37218 * t3675 - 0.42e1_f64 * t11145 * t9760 - 0.21e1_f64 * t3413 * t31929 - 0.42e1_f64 * t41028 * t2438 - 0.21e1_f64 * t12002 * t6751 - 0.105e1_f64 * t855 * t41033 * t352 - 0.63e1_f64 * t12009 * t8481 - 0.945e1_f64 * t41039 * t8481 - 0.315e1_f64 * t41042 * t2438 - 0.1575e1_f64 * t12009 * t6751 - 0.23625e1_f64 * t41047 * t8481 - 0.1575e1_f64 * t37199 * t3675 - 0.315e1_f64 * t11157 * t9760;
    t41054
}
