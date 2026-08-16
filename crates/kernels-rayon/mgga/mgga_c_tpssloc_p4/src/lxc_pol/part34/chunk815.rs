//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 815/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk815(t5465: f64, t626: f64, t5489: f64, t6320: f64, t67: f64, t758: f64, t750: f64, t17: f64, t588: f64, t6328: f64, t592: f64, t3701: f64, t6463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19471 = t626 * t5465;
    let t19480 = t626 * t5489;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19591 = t588 * t6328;
    let t19593 = t592 * t6328;
    let t19596 = t6463 * t3701;
    (t19471, t19480, t19542, t19576, t19591, t19593, t19596)
}
