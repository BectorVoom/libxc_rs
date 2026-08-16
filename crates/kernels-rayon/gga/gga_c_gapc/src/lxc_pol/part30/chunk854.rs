//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 854/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk854(t329: f64, t6210: f64, t2440: f64, t3238: f64, t3239: f64, t7029: f64, t2674: f64, t282: f64, t61: f64, t2255: f64, t3188: f64, t770: f64, t791: f64) -> (f64, f64, f64, f64, f64) {
    let t10203 = t6210 * t329;
    let t10204 = t10203 * t2440;
    let t10205 = t3238 * t10204;
    let t10207 = t3239 * t7029;
    let t10208 = t3238 * t10207;
    let t10210 = t2674 * t282;
    let t10211 = t61 * t10210;
    let t10212 = t3188 * t2255;
    let t10213 = t10211 * t10212;
    let t10215 = t791 * t770;
    (t10203, t10205, t10208, t10213, t10215)
}
