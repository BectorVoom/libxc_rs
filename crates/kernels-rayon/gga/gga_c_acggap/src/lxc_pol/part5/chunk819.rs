//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 819/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk819(t1922: f64, t377: f64, t407: f64, t6482: f64, t1539: f64, t6465: f64, t1160: f64, t6461: f64, t1411: f64, t1629: f64, t1533: f64, t1907: f64, t394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6529 = t377 * t1922;
    let t6532 = t6482 * t407;
    let t6535 = t6465 * t1539;
    let t6536 = t1160 * t6535;
    let t6538 = t6461 * t407;
    let t6541 = t1629 * t1411;
    let t6544 = t6465 * t407;
    let t6547 = t6461 * t1533;
    let t6551 = t394 * t1907;
    (t6529, t6532, t6535, t6536, t6538, t6541, t6544, t6547, t6551)
}
