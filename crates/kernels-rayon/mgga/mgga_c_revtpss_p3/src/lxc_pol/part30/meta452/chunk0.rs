//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1721/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1721(t3172: f64, t5303: f64, t1261: f64, t17633: f64, t5352: f64, t3720: f64, t1209: f64, t489: f64, t3623: f64, t370: f64, t1214: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17720 = t3172 * t5303;
    let t17721 = t1261 * t17720;
    let t17723 = t17633 * t5352;
    let t17724 = t3720 * t17723;
    let t17727 = t1209 * t489;
    let t17728 = t3623 * t370;
    let t17729 = t17727 * t17728;
    let t17730 = t606 * t1214;
    (t17720, t17721, t17724, t17727, t17728, t17729, t17730)
}
