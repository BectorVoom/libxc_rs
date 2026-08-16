//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1046/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1046(t3019: f64, t3057: f64, t3060: f64, t2915: f64, t2991: f64, t3016: f64, t375: f64, t2848: f64, t2854: f64, t209: f64, t2139: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26196 = t3019 * t3019;
    let t26197 = 1.0_f64 / t26196;
    let t26213 = t3057 * t3057;
    let t26214 = 1.0_f64 / t26213;
    let t26216 = t3060 * t3060;
    let t26217 = 1.0_f64 / t26216;
    let t26224 = 1.0_f64 / t3057 / t2915;
    let t26248 = t375 / t3016 / t2991;
    let t26255 = 1.0_f64 / t2848 / t2854;
    let t26261 = t209 * t2139 * t371;
    (t26197, t26214, t26217, t26224, t26248, t26255, t26261)
}
