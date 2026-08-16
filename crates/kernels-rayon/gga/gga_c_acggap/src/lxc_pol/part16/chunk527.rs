//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 527/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk527(t160: f64, t413: f64, t168: f64, t1160: f64, t1167: f64, t1162: f64, t3077: f64, t1159: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3370 = t160 * t413;
    let t3371 = t3370 * t168;
    let t3372 = t1160 * t3371;
    let t3373 = t3372 * t1167;
    let t3375 = t3077 * t1162;
    let t3376 = t3375 * t1167;
    let t3378 = t310 * t1159;
    (t3370, t3371, t3372, t3373, t3375, t3376, t3378)
}
