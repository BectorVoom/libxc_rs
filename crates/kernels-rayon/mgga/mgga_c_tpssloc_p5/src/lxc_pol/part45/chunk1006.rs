//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1006/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1006(t115305: f64, t6897: f64, t80645: f64, t8621: f64, t22633: f64, t31550: f64, t80650: f64, t22635: f64, t26331: f64, t31549: f64, t3734: f64, t22704: f64, t31559: f64, t81326: f64) -> (f64, f64, f64, f64, f64) {
    let t115306 = 0.63969658155208805863e-1_f64 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    let t115311 = t22633 * t80650 * t31550;
    let t115315 = t26331 * t22635 * t31549 * t3734;
    let t115318 = t22704 * t81326 * t31559;
    (t115306, t115308, t115311, t115315, t115318)
}
