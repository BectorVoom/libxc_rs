//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2034/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2034(t2635: f64, t81886: f64, t23041: f64, t2681: f64, t22690: f64, t23122: f64, t2553: f64, t841: f64, t22813: f64, t6589: f64, t80782: f64, t23124: f64) -> (f64, f64, f64, f64, f64) {
    let t81887 = t81886 * t2635;
    let t81889 = t23041 * t2681;
    let t81899 = t23122 * t22690 * t841 * t2553;
    let t81902 = t22813 * t6589 * t80782;
    let t81903 = t81902 * t23124;
    (t81887, t81889, t81899, t81902, t81903)
}
