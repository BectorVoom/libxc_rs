//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1256/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1256(t28342: f64, t28372: f64, t94229: f64, t28461: f64, t7904: f64, t1014: f64, t28528: f64, t54162: f64, t8147: f64, t2237: f64, t15815: f64, t303: f64, t7931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98515 = t28372 * t28342 * t94229;
    let t98519 = 0.46336805555555555556e-3_f64 * t28461 * t7904;
    let t98522 = t1014 * t28528;
    let t98524 = t54162 * t8147;
    let t98525 = t2237 * t98524;
    let t98528 = t303 * t7931 * t15815;
    (t98515, t98519, t98522, t98524, t98525, t98528)
}
