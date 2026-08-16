//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1002/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1002(t33272: f64, t81228: f64, t81326: f64, t33240: f64, t6883: f64, t115545: f64, t26331: f64, t26333: f64, t26189: f64, t31611: f64, t6888: f64, t115352: f64, t22892: f64, t7691: f64) -> (f64, f64, f64, f64, f64) {
    let t122281 = t81228 * t81326 * t33272;
    let t122295 = t6883 * t33240;
    let t122304 = t26331 * t115545 * t26333;
    let t122328 = t6888 * t31611 * t26189;
    let t122331 = t22892 * t115352 * t7691;
    (t122281, t122295, t122304, t122328, t122331)
}
