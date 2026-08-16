//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 791/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk791(t232: f64, t25236: f64, t6646: f64, t1888: f64, t13384: f64, t23110: f64, t7524: f64, t23185: f64, t234: f64, t6604: f64, t1484: f64, t252: f64) -> (f64, f64, f64, f64, f64) {
    let t25237 = t25236 * t232;
    let t25238 = t6646 * t25237;
    let t25239 = t1888 * t25238;
    let t25241 = t13384 * t232;
    let t25242 = t6646 * t25241;
    let t25243 = t1888 * t25242;
    let t25245 = t23110 * t7524;
    let t25246 = t23185 * t25245;
    let t25248 = t6604 * t234;
    let t25249 = t252 * t1484;
    (t25239, t25243, t25246, t25248, t25249)
}
