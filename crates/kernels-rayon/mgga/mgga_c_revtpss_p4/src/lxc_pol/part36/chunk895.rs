//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 895/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk895(t5940: f64, t705: f64, t2411: f64, t6079: f64, t5944: f64, t750: f64, t189: f64, t5825: f64, t212: f64, t6041: f64, t780: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t18263 = t705 * t5940;
    let t18268 = t6079 * t2411;
    let t18301 = t5944 * t750;
    let t18305 = t189 * t5825;
    let t18316 = t212 * t6041;
    let t18317 = t18316 * t780;
    let t18318 = t689 * t18317;
    (t18263, t18268, t18301, t18305, t18318)
}
