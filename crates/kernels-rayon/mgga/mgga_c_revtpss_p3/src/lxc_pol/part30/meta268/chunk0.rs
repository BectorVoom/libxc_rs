//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1180/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1180(t532: f64, t7311: f64, t1450: f64, t2014: f64, t1448: f64, t4147: f64, t2034: f64, t1459: f64, t2042: f64, t116: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7312 = t532 * t7311;
    let t7313 = t7312 * t1450;
    let t7314 = t2014 * t7313;
    let t7315 = t4147 * t1448;
    let t7316 = t2034 * t7315;
    let t7317 = t2014 * t7316;
    let t7329 = 3.0_f64 * t1459 * t2042;
    let t7330 = t116 * t1936;
    (t7312, t7313, t7314, t7315, t7316, t7317, t7329, t7330)
}
