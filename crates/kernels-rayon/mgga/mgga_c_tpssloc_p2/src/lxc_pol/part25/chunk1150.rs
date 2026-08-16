//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1150/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1150(t23102: f64, t80782: f64, t23113: f64, t10016: f64, t1898: f64, t249: f64, t23093: f64, t281: f64, t23046: f64, t812: f64, t835: f64, t2635: f64) -> (f64, f64, f64, f64) {
    let t81876 = t23102 * t80782;
    let t81877 = t81876 * t23113;
    let t81880 = t10016 * t1898 * t249;
    let t81882 = t23093 * t281;
    let t81883 = t81882 * t23113;
    let t81886 = t812 * t23046 * t835;
    let t81887 = t81886 * t2635;
    (t81877, t81880, t81883, t81887)
}
