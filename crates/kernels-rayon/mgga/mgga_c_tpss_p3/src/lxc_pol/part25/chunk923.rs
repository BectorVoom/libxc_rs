//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 923/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk923(t512: f64, t8186: f64, t1206: f64, t3260: f64, t1220: f64, t339: f64, t790: f64, t3346: f64, t72: f64, t240: f64, t3243: f64, t756: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10104 = 455.0_f64 / 1296.0_f64 * t8186 * t512;
    let t10106 = t3260 * t1206;
    let t10117 = t339 * t1220 * t790;
    let t10120 = t3346 * t72;
    let t10121 = t10120 * t240;
    let t10137 = t756 * t3243;
    (t10104, t10106, t10117, t10120, t10121, t10137)
}
