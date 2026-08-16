//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 773/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk773(t135: f64, t9105: f64, t4082: f64, t4085: f64, t1247: f64, t2282: f64, t12399: f64, t467: f64, t29976: f64, t4261: f64, t9074: f64, t19532: f64, t30136: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39644 = t135 * t9105;
    let t39646 = t4082 * t39644 * t4085;
    let t39648 = t1247 * t2282;
    let t39650 = t12399 * t467;
    let t39671 = t9074 * t4261 * t29976;
    let t39674 = t9074 * t19532 * t30136;
    (t39644, t39646, t39648, t39650, t39671, t39674)
}
