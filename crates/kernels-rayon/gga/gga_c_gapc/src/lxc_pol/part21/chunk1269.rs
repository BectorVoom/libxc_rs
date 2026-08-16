//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1269/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1269(t11395: f64, t5: f64, t25708: f64, t4055: f64, t8452: f64, t11261: f64, t13337: f64, t1416: f64, t1672: f64, t1: f64, t516: f64, t619: f64, t6803: f64, t8379: f64) -> (f64, f64, f64, f64) {
    let t35682 = t5 * t11395;
    let t35685 = t35682 * t25708 * t8452 * t4055;
    let t35689 = t11261 * t1416 * t1672 * t13337;
    let t35694 = t8379 * t516 * t1 * t6803 * t619;
    (t35682, t35685, t35689, t35694)
}
