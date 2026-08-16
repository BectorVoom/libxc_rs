//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 902/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk902(t41698: f64, t38051: f64, t544: f64, t9287: f64, t13398: f64, t7014: f64, t11172: f64, t2464: f64, t2465: f64, t2487: f64, t11386: f64, t2437: f64) -> (f64, f64, f64, f64, f64) {
    let t46176 = 0.20449560508757733161e1_f64 * t41698;
    let t46189 = t544 * t38051 * t9287;
    let t46190 = 0.14896037479937677779e-1_f64 * t46189;
    let t46191 = t7014 * t13398;
    let t46195 = t2487 * t2464 * t2465 * t11172;
    let t46212 = 0.35750489951850426669e0_f64 * t2437 * t11386;
    (t46176, t46190, t46191, t46195, t46212)
}
