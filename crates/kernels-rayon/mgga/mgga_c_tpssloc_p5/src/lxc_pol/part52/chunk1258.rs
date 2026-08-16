//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1258/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1258(t31203: f64, t6914: f64, t31207: f64, t6883: f64, t22724: f64, t31198: f64, t22704: f64, t22705: f64, t31202: f64, t1338: f64, t31181: f64, t22674: f64, t31123: f64, t6897: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114105 = t6914 * t31203;
    let t114116 = t6883 * t31207;
    let t114119 = 0.52089578783527170489e-1_f64 * t22724 * t31198;
    let t114121 = t22704 * t22705 * t31202;
    let t114130 = t1338 * t31181;
    let t114154 = t6897 * t22674 * t31123;
    (t114105, t114116, t114119, t114121, t114130, t114154)
}
