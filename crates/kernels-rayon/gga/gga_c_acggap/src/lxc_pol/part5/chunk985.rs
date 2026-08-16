//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 985/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk985(t1476: f64, t3143: f64, t1049: f64, t4833: f64, t4823: f64, t4819: f64, t13698: f64, t4811: f64, t1072: f64, t1298: f64, t3124: f64, t3126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16253 = t3143 * t1476;
    let t16255 = t1049 * t4833;
    let t16264 = t1049 * t4823;
    let t16274 = t1049 * t4819;
    let t16288 = t13698 * t4811;
    let t16292 = t3124 * t1072 * t1298 * t3126;
    (t16253, t16255, t16264, t16274, t16288, t16292)
}
