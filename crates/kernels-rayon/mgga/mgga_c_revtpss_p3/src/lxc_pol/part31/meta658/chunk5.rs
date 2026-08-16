//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2227/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2227(t5816: f64, t640: f64, t77: f64, t29561: f64, t644: f64, t4241: f64, t7705: f64, t1927: f64, t1926: f64, t101219: f64, t101227: f64, t101237: f64, t101240: f64, t101243: f64, t25157: f64, t28090: f64, t28151: f64, t28154: f64, t29562: f64, t7709: f64, t92568: f64, t92684: f64, t92687: f64, t92690: f64) -> f64 {
    let t108864 = t77 * t640 * t5816;
    let t108872 = t77 * t29561 * t644;
    let t108876 = t77 * t7705 * t4241;
    let t108879 = t1927 * t5816;
    let t108880 = t1926 * t108879;
    let t108889 = 2.0_f64 / 3.0_f64 * t7709 * t28090 - 5.0_f64 * t92684 * t29562 - 5.0_f64 * t92687 * t29562 - 5.0_f64 * t25157 * t108864 - 10.0_f64 / 3.0_f64 * t28154 * t101219 - 10.0_f64 / 3.0_f64 * t28154 * t101227 + 35.0_f64 * t92690 * t108872 - 10.0_f64 * t25157 * t108876 + 10.0_f64 * t92568 * t108880 - 10.0_f64 / 3.0_f64 * t101237 * t28151 - 10.0_f64 / 3.0_f64 * t101240 * t28151 - 10.0_f64 / 3.0_f64 * t101243 * t28151;
    t108889
}
