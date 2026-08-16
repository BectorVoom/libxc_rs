//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 699/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk699(t603: f64, t6632: f64, t6407: f64, t6424: f64, t6427: f64, t601: f64, t1986: f64, t1998: f64, t580: f64, t587: f64, t6419: f64, t1994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6633 = t6632 * t603;
    let t6634 = 0.17544670192365612213e1_f64 * t6633;
    let t6636 = t6424 * t6407 * t6427;
    let t6638 = 0.1025389702100779493e4_f64 * t601 * t6636;
    let t6639 = t1986 * t1998;
    let t6640 = 0.17544670192365612213e1_f64 * t6639;
    let t6642 = t580 * t6419 * t587;
    let t6644 = 0.58482233974552040708e0_f64 * t601 * t6642;
    let t6646 = t1986 * t1994;
    (t6634, t6636, t6638, t6640, t6642, t6644, t6646)
}
