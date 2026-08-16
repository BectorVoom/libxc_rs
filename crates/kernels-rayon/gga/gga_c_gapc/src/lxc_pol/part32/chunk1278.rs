//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1278/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1278(t11214: f64, t11663: f64, t6853: f64, t760: f64, t10343: f64, t3734: f64, t10336: f64, t291: f64, t640: f64, t3243: f64, t6188: f64, t10287: f64, t11648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35745 = t11214 * t760 * t6853 * t11663;
    let t35747 = t10343 * t3734;
    let t35749 = t10336 * t3734;
    let t35751 = t640 * t291;
    let t35753 = t3243 * t35751 * t6188;
    let t35755 = t10287 * t11648;
    (t35745, t35747, t35749, t35751, t35753, t35755)
}
