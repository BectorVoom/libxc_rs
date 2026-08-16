//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1808/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1808(t213: f64, t6589: f64, t9223: f64, t6593: f64, t23062: f64, t23066: f64, t22715: f64, t229: f64, t805: f64, t1891: f64, t192: f64, t22690: f64, t80881: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81933 = t9223 * t6589 * t213;
    let t81934 = t81933 * t6593;
    let t81936 = t23062 * t23066;
    let t81942 = t22715 * t229;
    let t81943 = t81942 * t805;
    let t81954 = t80881 * t1891 * t192 * t22690 * t841;
    (t81933, t81934, t81936, t81942, t81943, t81954)
}
