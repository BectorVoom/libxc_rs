//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 956/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk956(t10863: f64, t10866: f64, t10901: f64, t11017: f64, t11002: f64, t1115: f64, t792: f64, t2867: f64, t481: f64, t3574: f64, t2333: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11432 = 0.28914548798370980346e-3_f64 * t10863;
    let t11433 = 0.42683466926433871473e0_f64 * t10866;
    let t11444 = 0.45022119329691164871e0_f64 * t10901;
    let t11454 = 0.39032073591371545778e-3_f64 * t11017;
    let t11465 = t11002 * t1115 * t792;
    let t11475 = t2867 * t481;
    let t11486 = t3574 * t792;
    let t11496 = t2333 * t910;
    (t11432, t11433, t11444, t11454, t11465, t11475, t11486, t11496)
}
