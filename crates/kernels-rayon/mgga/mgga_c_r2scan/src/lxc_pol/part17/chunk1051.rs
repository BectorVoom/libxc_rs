//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1051/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1051(t818: f64, t9638: f64, t10533: f64, t856: f64, t352: f64, t9769: f64, t910: f64, t986: f64, t113: f64, t5086: f64, t104: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31764 = t9638 * t818;
    let t35213 = t10533 * t856;
    let t35220 = t352 * t9769;
    let t35373 = t986 * t910;
    let t36967 = t113 * t5086;
    let t36985 = t104 * t494;
    (t31764, t35213, t35220, t35373, t36967, t36985)
}
