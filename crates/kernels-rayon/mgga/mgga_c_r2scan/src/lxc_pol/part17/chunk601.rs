//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 601/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk601(t3319: f64, t3320: f64, t783: f64, t1060: f64, t560: f64, t1058: f64, t2201: f64, t481: f64, t2207: f64, t1059: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3322 = t783 * t3319 * t3320;
    let t3324 = t1060 * t560;
    let t3326 = t2201 * t1058 * t3324;
    let t3328 = t1060 * t481;
    let t3330 = t2207 * t1058 * t3328;
    let t3332 = t269 * t1059;
    (t3322, t3324, t3326, t3328, t3330, t3332)
}
