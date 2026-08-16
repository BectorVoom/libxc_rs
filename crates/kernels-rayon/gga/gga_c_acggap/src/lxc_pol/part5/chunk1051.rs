//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1051/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1051(t1137: f64, t4583: f64, t4587: f64, t14173: f64, t4741: f64, t1111: f64, t1165: f64, t16375: f64, t3391: f64, t3529: f64, t4417: f64, t3759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18366 = t1137 * t4583;
    let t18368 = t1137 * t4587;
    let t18388 = t14173 * t4741;
    let t18392 = t3391 * t1165 * t16375 * t1111;
    let t18396 = t3391 * t1165 * t4417 * t3529;
    let t18400 = t3391 * t1165 * t4417 * t3759;
    (t18366, t18368, t18388, t18392, t18396, t18400)
}
