//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 789/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk789(t271: f64, t2775: f64, t974: f64, t2769: f64, t632: f64, t344: f64, t9288: f64, t698: f64, t976: f64, t979: f64, t973: f64, t2970: f64, t2999: f64) -> (f64, f64, f64, f64, f64) {
    let t10213 = 1.0_f64 / t271 / t2775;
    let t10214 = t974 * t10213;
    let t10216 = 1.0_f64 / t2769 / t632;
    let t10217 = t344 * t10216;
    let t10218 = t10217 * t9288;
    let t10219 = t10214 * t10218;
    let t10224 = t698 * t976;
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10228 = t2970 * t2999;
    (t10213, t10216, t10219, t10226, t10228)
}
