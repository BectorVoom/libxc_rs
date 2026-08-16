//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 996/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk996(t18197: f64, t5238: f64, t5236: f64, t115: f64, t17926: f64, t5: f64, t497: f64, t4300: f64, t5096: f64, t4299: f64, t1554: f64, t5087: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18198 = 1.0_f64 / t18197;
    let t18199 = t5238 * t18198;
    let t18200 = t5236 * t18199;
    let t18204 = t17926 * t115 * t5;
    let t18205 = t18204 * t497;
    let t18213 = t4300 * t5096;
    let t18214 = t4299 * t18213;
    let t18218 = t5087 * t1554;
    (t18198, t18199, t18200, t18204, t18205, t18213, t18214, t18218)
}
