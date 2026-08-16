//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 783/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk783(t29976: f64, t4261: f64, t9074: f64, t19532: f64, t30136: f64, t2321: f64, t30334: f64, t12424: f64, t2312: f64, t12427: f64, t484: f64, t29854: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39671 = t9074 * t4261 * t29976;
    let t39674 = t9074 * t19532 * t30136;
    let t39677 = t9074 * t30334 * t2321;
    let t39679 = t2312 * t12424;
    let t39681 = t2312 * t12427;
    let t39695 = t484 * t12424;
    let t39717 = t9074 * t4261 * t29854;
    (t39671, t39674, t39677, t39679, t39681, t39695, t39717)
}
