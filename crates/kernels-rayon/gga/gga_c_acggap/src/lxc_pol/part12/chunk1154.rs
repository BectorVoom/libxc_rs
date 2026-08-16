//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1154/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1154(t1662: f64, t1679: f64, t8040: f64, t9461: f64, t1298: f64, t694: f64, t8034: f64, t2147: f64, t2394: f64, t7885: f64, t864: f64, t315: f64, t5386: f64, t634: f64) -> (f64, f64, f64, f64, f64) {
    let t36769 = 2.0_f64 * t1679 * t8040 * t1662;
    let t36771 = 4.0_f64 * t1679 * t9461;
    let t36774 = 6.0_f64 * t694 * t8034 * t1298;
    let t36794 = t7885 * t2147 * t2394 * t864;
    let t36808 = 0.26341796731742046394e1_f64 * t315 * t634 * t5386;
    (t36769, t36771, t36774, t36794, t36808)
}
