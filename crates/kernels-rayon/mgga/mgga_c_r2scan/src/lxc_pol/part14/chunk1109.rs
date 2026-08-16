//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1109/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1109(t1053: f64, t1102: f64, t1103: f64, t7028: f64, t1563: f64, t3582: f64, t11496: f64, t2262: f64, t6967: f64, t7040: f64, t792: f64, t11670: f64, t8098: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39309 = t1102 * t1053 * t1103 * t7028;
    let t39318 = t3582 * t1563;
    let t39327 = t11496 * t2262;
    let t39335 = t6967 * t2262;
    let t39339 = t7040 * t792;
    let t39352 = t11670 * t8098;
    (t39309, t39318, t39327, t39335, t39339, t39352)
}
