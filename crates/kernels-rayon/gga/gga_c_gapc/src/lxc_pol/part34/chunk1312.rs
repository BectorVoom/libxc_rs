//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1312/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1312(t10343: f64, t3734: f64, t10336: f64, t291: f64, t640: f64, t3243: f64, t6188: f64, t10287: f64, t11648: f64, t24625: f64, t3643: f64, t11270: f64, t24398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35747 = t10343 * t3734;
    let t35749 = t10336 * t3734;
    let t35751 = t640 * t291;
    let t35753 = t3243 * t35751 * t6188;
    let t35755 = t10287 * t11648;
    let t35759 = t3643 * t24625 * t3734;
    let t35762 = t11270 * t24398 * t11648;
    (t35747, t35749, t35751, t35753, t35755, t35759, t35762)
}
