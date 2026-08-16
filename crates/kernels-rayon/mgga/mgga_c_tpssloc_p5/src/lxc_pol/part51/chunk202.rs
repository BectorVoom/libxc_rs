//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 202/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk202(t123: f64, t67: f64, t687: f64, t3: f64, t61: f64, t119: f64, t133: f64) -> (f64, f64, f64, f64) {
    let t692 = f64::sqrt(t123);
    let t693 = t692 * t67;
    let t694 = t693 * t687;
    let t697 = 1.0_f64 / t61 / t3;
    let t698 = t119 * t697;
    let t699 = t133 * t698;
    (t693, t694, t697, t699)
}
