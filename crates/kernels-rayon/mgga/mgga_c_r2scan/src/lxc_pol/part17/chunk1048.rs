//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1048/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1048(t560: f64, t8825: f64, t481: f64, t8783: f64, t113: f64, t8701: f64, t2530: f64, t921: f64, t2182: f64, t979: f64, t3071: f64, t6212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30281 = t8825 * t560;
    let t30285 = t8825 * t481;
    let t30292 = t8783 * t560;
    let t30296 = t8783 * t481;
    let t30304 = t8701 * t113;
    let t30320 = t921 * t2530;
    let t30370 = t2182 * t979;
    let t30428 = t6212 * t3071;
    (t30281, t30285, t30292, t30296, t30304, t30320, t30370, t30428)
}
