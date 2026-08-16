//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 921/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk921(t7293: f64, t9094: f64, t2594: f64, t8968: f64, t11701: f64, t5218: f64, t2568: f64, t9016: f64, t2576: f64, t9079: f64, t28324: f64, t5290: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29356 = 3.0_f64 * t7293 * t9094;
    let t29357 = t8968 * t2594;
    let t29359 = 6.0_f64 * t11701 * t29357;
    let t29360 = t2594 * t9094;
    let t29362 = 6.0_f64 * t5218 * t29360;
    let t29363 = t9016 * t2568;
    let t29365 = t2576 * t9079;
    let t29367 = t5290 * t28324;
    (t29356, t29359, t29362, t29363, t29365, t29367)
}
