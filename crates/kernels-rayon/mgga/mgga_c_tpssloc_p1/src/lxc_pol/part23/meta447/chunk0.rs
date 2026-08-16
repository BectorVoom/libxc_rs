//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1292/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1292(t12250: f64, t6414: f64, t1338: f64, t20601: f64, t12461: f64, t20684: f64, t571: f64, t6330: f64, t20193: f64, t604: f64, t1409: f64, t1426: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75008 = t12250 * t6414;
    let t75124 = t1338 * t20601;
    let t75240 = t20684 * t12461;
    let t75256 = t6330 * t571;
    let t75284 = t20193 * t604;
    let t75361 = t1409 * t1426 * t67;
    (t75008, t75124, t75240, t75256, t75284, t75361)
}
