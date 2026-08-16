//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1056/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1056(t12752: f64, t1562: f64, t3382: f64, t4439: f64, t1347: f64, t3237: f64, t4932: f64, t997: f64, t1418: f64, t5260: f64, t1165: f64, t3491: f64, t4282: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18555 = t12752 * t1562;
    let t18566 = t3382 * t4439;
    let t18578 = t3237 * t1347;
    let t18580 = t997 * t4932;
    let t18582 = t3237 * t1418;
    let t18584 = t997 * t5260;
    let t18588 = t4282 * t1165 * t540 * t3491;
    (t18555, t18566, t18578, t18580, t18582, t18584, t18588)
}
