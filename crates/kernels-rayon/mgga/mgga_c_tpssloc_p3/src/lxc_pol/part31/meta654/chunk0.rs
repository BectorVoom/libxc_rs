//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1934/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1934(t23097: f64, t232: f64, t67783: f64, t815: f64, t16888: f64, t23146: f64, t16969: f64, t25146: f64, t4236: f64, t23053: f64, t5614: f64, t16859: f64, t6614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98672 = t23097 * t815 * t67783 * t232;
    let t98674 = t23146 * t16888;
    let t98676 = t23146 * t16969;
    let t98678 = t25146 * t4236;
    let t98680 = t23053 * t5614;
    let t98682 = t6614 * t16859;
    (t98672, t98674, t98676, t98678, t98680, t98682)
}
