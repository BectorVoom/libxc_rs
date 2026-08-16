//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 790/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk790(t2771: f64, t6690: f64, t23593: f64, t3034: f64, t38: f64, t131: f64, t350: f64, t3030: f64, t344: f64, t1014: f64, t1011: f64, t360: f64) -> (f64, f64, f64, f64, f64) {
    let t23594 = t6690 * t2771;
    let t23595 = t23593 * t23594;
    let t23598 = 1.0_f64 / t3034;
    let t23599 = t38 * t23598;
    let t23600 = t23599 * t131;
    let t23601 = t23600 * t350;
    let t23602 = t344 * t3030;
    let t23603 = t23602 * t1014;
    let t23604 = t1011 * t360;
    (t23595, t23601, t23602, t23603, t23604)
}
