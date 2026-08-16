//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1111/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1111(t2262: f64, t6967: f64, t3263: f64, t3275: f64, t7040: f64, t792: f64, t3276: f64, t11020: f64, t11540: f64, t10622: f64, t11629: f64, t3579: f64, t38283: f64) -> (f64, f64, f64, f64, f64) {
    let t39335 = t6967 * t2262;
    let t39338 = t3275 * t3263 * t39335 / 2.0_f64;
    let t39339 = t7040 * t792;
    let t39342 = 5.0_f64 / 8.0_f64 * t3275 * t3276 * t39339;
    let t39344 = t11020 * t11540 / 4.0_f64;
    let t39347 = 5.0_f64 / 16.0_f64 * t3275 * t11629 * t10622;
    let t39351 = t3579 * t38283 / 4.0_f64;
    (t39338, t39342, t39344, t39347, t39351)
}
