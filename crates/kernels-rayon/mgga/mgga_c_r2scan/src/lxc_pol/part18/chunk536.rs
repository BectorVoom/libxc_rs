//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 536/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk536(t2910: f64, t1000: f64, t1256: f64, t2904: f64, t308: f64, t1001: f64, t1268: f64, t2901: f64, t2905: f64, t295: f64, t305: f64, t309: f64, t997: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2911 = tau1 * t2910;
    let t2916 = t1000 * t1000;
    let t2917 = t1256 * t2916;
    let t2920 = -t2904;
    let t2921 = t308 * t2920;
    let t2924 = 10.0_f64 / 9.0_f64 * t295 * t2901 + 5.0_f64 / 3.0_f64 * t295 * t2905 + 40.0_f64 / 9.0_f64 * t2911 * t309 - 50.0_f64 / 9.0_f64 * t997 * t1001 + 10.0_f64 / 9.0_f64 * t305 * t2917 + 5.0_f64 / 3.0_f64 * t305 * t2921 - t1268;
    (t2911, t2916, t2917, t2920, t2921, t2924)
}
