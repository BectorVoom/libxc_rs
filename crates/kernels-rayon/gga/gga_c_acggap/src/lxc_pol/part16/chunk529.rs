//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 529/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk529(t1162: f64, t3360: f64, t1172: f64, t2450: f64, t1024: f64, t134: f64, t1161: f64, t1170: f64, t3371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3391 = t3360 * t1162;
    let t3396 = t2450 * t1172;
    let t3401 = t134 * t1024;
    let t3402 = t1161 * t3401;
    let t3403 = t1170 * t3402;
    let t3409 = t1170 * t3371;
    (t3391, t3396, t3401, t3402, t3403, t3409)
}
