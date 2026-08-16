//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 867/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk867(t2305: f64, t35326: f64, t7371: f64, t8577: f64, t39277: f64, t7234: f64, t7239: f64, t7733: f64, t16043: f64, t9111: f64, t2283: f64, t35277: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39308 = t35326 * t2305;
    let t39310 = t8577 * t7371;
    let t39312 = t39277 * t7234;
    let t39314 = t39277 * t7239;
    let t39316 = t39277 * t7733;
    let t39323 = t16043 * t9111;
    let t39325 = t35277 * t2283;
    (t39308, t39310, t39312, t39314, t39316, t39323, t39325)
}
