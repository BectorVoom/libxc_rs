//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 653/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk653(t72: f64, t7431: f64, t1410: f64, t605: f64, t1433: f64, t71: f64, t1458: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t7432 = t72 * t7431;
    let t7435 = t605 * t1410;
    let t7445 = t71 * t1433;
    let t7458 = t89 * t1458;
    (t7432, t7435, t7445, t7458)
}
