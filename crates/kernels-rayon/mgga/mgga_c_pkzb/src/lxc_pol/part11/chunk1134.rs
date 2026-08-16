//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1134/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1134(t179: f64, t18199: f64, t299: f64, t3542: f64, t2099: f64, t2945: f64, t9590: f64, t154: f64, t2048: f64, t276: f64, t9161: f64, t300: f64, t3638: f64, t779: f64) -> (f64, f64, f64, f64) {
    let t25248 = t299 * t179 * t18199 * t3542;
    let t25275 = t2945 * t2099 * t9590;
    let t25290 = t276 * t154 * t2048 * t9161;
    let t25337 = t300 * t779 * t3638;
    (t25248, t25275, t25290, t25337)
}
