//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1277/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1277(t24625: f64, t3643: f64, t3734: f64, t11270: f64, t11648: f64, t24398: f64, t10328: f64, t11688: f64, t23132: f64, t24081: f64, t17874: f64, t35469: f64) -> (f64, f64, f64, f64, f64) {
    let t35759 = t3643 * t24625 * t3734;
    let t35762 = t11270 * t24398 * t11648;
    let t35764 = t10328 * t11688;
    let t35766 = t24081 * t23132;
    let t35768 = t35766 * t35469 * t17874;
    (t35759, t35762, t35764, t35766, t35768)
}
