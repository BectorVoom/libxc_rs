//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1046/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1046(t10666: f64, t113: f64, t97: f64, t13908: f64, t795: f64, t3270: f64, t3347: f64, t5086: f64, t1064: f64, t23040: f64, t3348: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t37282 = t97 * t10666 * t113;
    let t37285 = t13908 * t795;
    let t37286 = t3270 * t37285;
    let t37292 = t5086 * t3347;
    let t37299 = t23040 * t1064;
    let t37312 = t3348 * t481;
    (t37282, t37286, t37292, t37299, t37312)
}
