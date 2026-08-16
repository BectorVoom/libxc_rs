//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 729/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk729(t1719: f64, t713: f64, t695: f64, t717: f64, t1800: f64, t632: f64, t645: f64, t190: f64, t5686: f64, t1898: f64, t650: f64, t1907: f64, t5448: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5798 = t713 * t1719;
    let t5801 = t717 * t695;
    let t5812 = 6.0_f64 * t632 * t645 * t1800;
    let t5815 = 2.0_f64 * t632 * t190 * t5686;
    let t5818 = 0.48245938496077605201e2_f64 * t650 * t1898 * t1800;
    let t5821 = 24.0_f64 * t1907 * t190 * t5448;
    (t5798, t5801, t5812, t5815, t5818, t5821)
}
