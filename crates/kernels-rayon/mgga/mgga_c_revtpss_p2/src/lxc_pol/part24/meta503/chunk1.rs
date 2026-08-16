//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1510/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1510(t14923: f64, t23301: f64, t125: f64, t23114: f64, t10777: f64, t10779: f64, t6035: f64, t61715: f64, t14931: f64, t23334: f64, t61956: f64, t10811: f64, t23331: f64) -> (f64, f64, f64, f64, f64) {
    let t76703 = t14923 * t23301;
    let t76705 = t125 * t23114;
    let t76720 = t10777 * t10779 * t61715 * t6035;
    let t76738 = t14931 * t10779 * t61956 * t23334;
    let t76740 = t10811 * t23331;
    (t76703, t76705, t76720, t76738, t76740)
}
