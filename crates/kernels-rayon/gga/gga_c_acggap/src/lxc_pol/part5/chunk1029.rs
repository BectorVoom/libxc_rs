//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1029/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1029(t12589: f64, t4376: f64, t4380: f64, t4396: f64, t4567: f64, t3382: f64, t4402: f64, t4894: f64, t997: f64, t1576: f64, t3228: f64, t1581: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17586 = t12589 * t4376;
    let t17592 = t4396 * t4380;
    let t17605 = t4396 * t4567;
    let t17607 = t3382 * t4402;
    let t17613 = t997 * t4894;
    let t17615 = t3228 * t1576;
    let t17617 = t3228 * t1581;
    (t17586, t17592, t17605, t17607, t17613, t17615, t17617)
}
