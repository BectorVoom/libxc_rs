//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 961/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk961(t1218: f64, t230: f64, t3260: f64, t520: f64, t3267: f64, t3334: f64, t512: f64, t8186: f64, t3326: f64, t1220: f64, t339: f64, t790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10084 = t1218 * t1218;
    let t10085 = 1.0_f64 / t10084;
    let t10086 = t10085 * t230;
    let t10089 = t3260 * t520;
    let t10100 = t3267 * t3334;
    let t10104 = 455.0_f64 / 1296.0_f64 * t8186 * t512;
    let t10111 = t3260 * t3326;
    let t10117 = t339 * t1220 * t790;
    (t10085, t10086, t10089, t10100, t10104, t10111, t10117)
}
