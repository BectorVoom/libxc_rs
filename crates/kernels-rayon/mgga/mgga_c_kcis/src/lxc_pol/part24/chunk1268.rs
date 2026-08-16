//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1268/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1268(t1134: f64, t303: f64, t6482: f64, t100284: f64, t100314: f64, t100389: f64, t100736: f64, t100741: f64, t100746: f64, t11020: f64, t20684: f64, t2197: f64, t26960: f64, t28113: f64, t28118: f64, t28123: f64, t28125: f64, t7779: f64, t96917: f64, t96926: f64) -> (f64, f64) {
    let t100749 = t303 * t6482 * t1134;
    let t100751 = 0.46336805555555555556e-3_f64 * t96917 * t28118 + 0.30918233506944444445e-4_f64 * t96926 * t28113 - 0.30891203703703703704e-3_f64 * t96917 * t28125 + 0.46336805555555555556e-3_f64 * t26960 * t100284 + 0.15445601851851851852e-3_f64 * t26960 * t11020 * t28123 * t100314 - 0.15476481481481481481e-2_f64 * t100736 + 0.15445601851851851852e-3_f64 * t26960 * t100389 - 0.51588271604938271603e-3_f64 * t100741 + 0.92673611111111111112e-3_f64 * t20684 * t7779 * t2197 - 0.11584201388888888889e-3_f64 * t100746 + 0.11607361111111111111e-2_f64 * t100749;
    (t100749, t100751)
}
