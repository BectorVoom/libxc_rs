//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1203/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1203(t1395: f64, t226: f64, t782: f64, t1379: f64, t818: f64, t5570: f64, t811: f64, t1706: f64, t10584: f64, t10579: f64, t1398: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19748 = t1395 * t782 * t226;
    let t19762 = t1379 * t818;
    let t19766 = t5570 * t811;
    let t19767 = t1706 * t19766;
    let t19769 = t10584 * t782;
    let t19781 = t10579 * t226;
    let t19809 = t1398 * t750;
    (t19748, t19762, t19766, t19767, t19769, t19781, t19809)
}
