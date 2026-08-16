//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 566/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk566(t1106: f64, t1181: f64, t423: f64, t3361: f64, t1111: f64, t1165: f64, t3189: f64, t160: f64, t413: f64, t168: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3363 = t1181 * t423 * t1106;
    let t3364 = t3361 * t3363;
    let t3367 = t1165 * t3189 * t1111;
    let t3368 = t3361 * t3367;
    let t3370 = t160 * t413;
    let t3371 = t3370 * t168;
    (t3363, t3364, t3367, t3368, t3370, t3371)
}
