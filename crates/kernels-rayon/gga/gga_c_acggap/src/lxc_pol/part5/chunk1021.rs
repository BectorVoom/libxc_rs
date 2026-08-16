//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1021/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1021(t3621: f64, t4579: f64, t4484: f64, t3382: f64, t4406: f64, t1095: f64, t1524: f64, t384: f64, t398: f64, t879: f64, t1529: f64, t848: f64) -> (f64, f64, f64, f64, f64) {
    let t17355 = t3621 * t4579;
    let t17357 = t3621 * t4484;
    let t17362 = t3382 * t4406;
    let t17371 = t384 * t398 * t1095 * t1524 * t879;
    let t17386 = t848 * t1529;
    (t17355, t17357, t17362, t17371, t17386)
}
