//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3166/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3166(t11818: f64, t1213: f64, t248: f64, t6219: f64, t3036: f64, t6163: f64, t3500: f64, t3503: f64, t1210: f64, t15734: f64, t5005: f64, t19047: f64, t3572: f64) -> (f64, f64, f64, f64, f64) {
    let t65528 = t1213 * t248 * t11818 * t6219;
    let t65539 = t6163 * t3036;
    let t65541 = t3500 * t3503 * t65539;
    let t65545 = t3500 * t1210 * t65539;
    let t65552 = t5005 * t15734;
    let t65554 = t19047 * t3572;
    (t65528, t65541, t65545, t65552, t65554)
}
