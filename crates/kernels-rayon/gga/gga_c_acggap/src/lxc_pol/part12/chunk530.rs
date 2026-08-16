//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 530/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk530(t1008: f64, t1200: f64, t1195: f64, t997: f64, t336: f64, t360: f64, t1017: f64, t322: f64, t1459: f64, t398: f64, t384: f64, t1016: f64, t141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3271 = t1008 * t1200;
    let t3273 = t1008 * t1195;
    let t3280 = t997 * t1200;
    let t3282 = t336 * t360;
    let t3290 = t1017 * t322;
    let t3292 = t398 * t1459 * t3290;
    let t3293 = t384 * t3292;
    let t3300 = t141 * t1016;
    (t3271, t3273, t3280, t3282, t3290, t3292, t3293, t3300)
}
