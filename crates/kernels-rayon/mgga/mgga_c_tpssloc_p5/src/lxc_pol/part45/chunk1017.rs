//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1017/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1017(t115454: f64, t115468: f64, t1985: f64, t1998: f64, t214: f64, t24063: f64, t114081: f64, t114085: f64, t114098: f64, t114102: f64, t114104: f64, t114106: f64, t115439: f64, t1332: f64, t1336: f64, t31636: f64, t31639: f64, t3856: f64, t544: f64, t553: f64) -> (f64, f64) {
    let t115469 = t115454 + t115468;
    let t115474 = t1985 * t214 * t1998 * t24063;
    let t115480 = t114081 - 0.82246703342411321824e-2_f64 * t115439 + t544 * t553 * t115469 + 0.82246703342411321825e-2_f64 * t115474 - t114085 - t1336 * t31636 * t3856 - t114098 + t114102 + t114104 + t114106 + 2.0_f64 * t1332 * t31639;
    (t115469, t115480)
}
