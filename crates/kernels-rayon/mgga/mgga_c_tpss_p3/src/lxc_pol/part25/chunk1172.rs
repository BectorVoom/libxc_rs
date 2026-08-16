//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1172/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1172(t517: f64, t5543: f64, t1215: f64, t1693: f64, t527: f64, t3255: f64, t64: f64, t234: f64, t339: f64, t5719: f64, t789: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18436 = t5543 * t517;
    let t18437 = t18436 * t1215;
    let t18438 = 7.0_f64 / 72.0_f64 * t18437;
    let t18439 = t1693 * t527;
    let t18444 = t3255 * t64;
    let t18446 = t339 * t18444 * t234;
    let t18450 = t339 * t5719 * t789;
    (t18436, t18438, t18439, t18444, t18446, t18450)
}
