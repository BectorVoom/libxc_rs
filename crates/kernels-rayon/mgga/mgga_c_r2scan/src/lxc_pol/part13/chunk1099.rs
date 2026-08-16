//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1099/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1099(t3275: f64, t3276: f64, t39178: f64, t6897: f64, t910: f64, t2330: f64, t3262: f64, t3263: f64, t11622: f64, t37271: f64, t3261: f64, t5086: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t39181 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t39178;
    let t39182 = t6897 * t910;
    let t39183 = t39182 * t2330;
    let t39186 = 3.0_f64 / 2.0_f64 * t3262 * t3263 * t39183;
    let t39188 = 45.0_f64 / 32.0_f64 * t37271 * t11622;
    let t39190 = t97 * t3261 * t5086;
    (t39181, t39186, t39188, t39190)
}
