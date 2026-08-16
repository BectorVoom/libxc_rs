//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1156/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1156(t10029: f64, t1614: f64, t3211: f64, t3214: f64, t1170: f64, t4430: f64, t1173: f64, t4377: f64, t724: f64, t489: f64, t10033: f64, t2215: f64, t4438: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12907 = 0.11696447245269292414e1_f64 * t10029;
    let t12908 = t3211 * t1614;
    let t12909 = 12.0_f64 * t12908;
    let t12910 = t3214 * t1614;
    let t12911 = 32.0_f64 * t12910;
    let t12913 = 8.0_f64 * t1170 * t4430;
    let t12915 = 8.0_f64 * t1173 * t4430;
    let t12916 = t4377 * t724;
    let t12918 = 2.0_f64 * t489 * t12916;
    let t12919 = 40.0_f64 * t10033;
    let t12920 = t4438 * t2215;
    (t12907, t12909, t12911, t12913, t12915, t12918, t12919, t12920)
}
