//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1085/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1085(t225: f64, t30247: f64, t26304: f64, t30105: f64, t1882: f64, t543: f64, t8085: f64, t7301: f64, t2097: f64, t6843: f64, t30225: f64, t6895: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30248 = t30247 * t225;
    let t30252 = t26304 * t30105;
    let t30256 = t8085 * t1882 * t543;
    let t30257 = t7301 * t30256;
    let t30261 = t2097 * t6843 * t543;
    let t30262 = t7301 * t30261;
    let t30266 = t30225 * t543;
    let t30267 = t7301 * t30266;
    let t30278 = t2097 * t6895;
    (t30248, t30252, t30256, t30257, t30261, t30262, t30266, t30267, t30278)
}
