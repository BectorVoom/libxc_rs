//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1099/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1099(t2642: f64, t5384: f64, t501: f64, t6825: f64, t1545: f64, t2607: f64, t16880: f64, t16882: f64, t2609: f64, t5342: f64, t16910: f64, t16929: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20274 = t5384 * t2642;
    let t20275 = 0.17006693853500995666e-1_f64 * t20274;
    let t20334 = t501 * t6825;
    let t20336 = t1545 * t2607;
    let t20337 = 36.0_f64 * t20336;
    let t20338 = 480.0_f64 * t16880;
    let t20339 = 96.0_f64 * t16882;
    let t20340 = t2609 * t5342;
    let t20349 = 8.0_f64 * t16910;
    let t20352 = 960.0_f64 * t16929;
    (t20275, t20334, t20337, t20338, t20339, t20340, t20349, t20352)
}
