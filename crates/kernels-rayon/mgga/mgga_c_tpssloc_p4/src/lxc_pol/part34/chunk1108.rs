//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1108/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1108(t23102: f64, t80782: f64, t23093: f64, t281: f64, t23046: f64, t812: f64, t835: f64, t22813: f64, t6589: f64, t23138: f64, t6604: f64, t22690: f64, t2627: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81876 = t23102 * t80782;
    let t81882 = t23093 * t281;
    let t81886 = t812 * t23046 * t835;
    let t81902 = t22813 * t6589 * t80782;
    let t81911 = t23138 * t6604;
    let t81914 = t22690 * t2627;
    (t81876, t81882, t81886, t81902, t81911, t81914)
}
