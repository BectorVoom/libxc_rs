//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1095/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1095(t10943: f64, t11603: f64, t2333: f64, t2847: f64, t10710: f64, t25480: f64, t37658: f64, t25486: f64, t37582: f64, t10776: f64, t10810: f64, t2563: f64) -> (f64, f64, f64, f64, f64) {
    let t39290 = t10943 * t11603;
    let t39299 = t2333 * t2847;
    let t39355 = t37658 * t10710 * t25480;
    let t39358 = t37582 * t10710 * t25486;
    let t39361 = t10776 * t10810 * t2563;
    (t39290, t39299, t39355, t39358, t39361)
}
