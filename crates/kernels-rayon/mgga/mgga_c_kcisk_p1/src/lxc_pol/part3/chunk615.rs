//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 615/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk615(t5298: f64, t735: f64, t734: f64, t4803: f64, t642: f64, t5174: f64, t716: f64, t740: f64, t748: f64, t4816: f64, t1950: f64, t1945: f64, t1954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5299 = t735 * t5298;
    let t5300 = t734 * t5299;
    let t5302 = t642 * t4803;
    let t5303 = t735 * t5302;
    let t5304 = t734 * t5303;
    let t5306 = t5174 * t716;
    let t5307 = t5306 * t740;
    let t5308 = t5307 * t748;
    let t5310 = t4816 * t740;
    let t5311 = t5310 * t1950;
    let t5313 = t1945 * t1954;
    (t5299, t5300, t5302, t5303, t5304, t5306, t5307, t5308, t5310, t5311, t5313)
}
