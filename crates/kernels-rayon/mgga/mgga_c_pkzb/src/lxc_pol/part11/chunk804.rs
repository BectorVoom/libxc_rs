//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 804/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk804(t2394: f64, t8359: f64, t1229: f64, t5939: f64, t918: f64, t2364: f64, t2029: f64, t3199: f64, t2376: f64, t3214: f64, t1238: f64, t2407: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8360 = t2394 * t8359;
    let t8363 = t5939 * t1229;
    let t8364 = t918 * t8363;
    let t8368 = t2364 * t8359;
    let t8380 = t3199 * t2029;
    let t8386 = 0.15244095330869239812e-2_f64 * t3214 * t2376;
    let t8389 = 0.30488190661738479624e-2_f64 * t1238 * t2407;
    (t8360, t8363, t8364, t8368, t8380, t8386, t8389)
}
