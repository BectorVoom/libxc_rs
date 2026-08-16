//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2254/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2254(t2247: f64, t4187: f64, t10309: f64, t1470: f64, t101227: f64, t101230: f64, t101234: f64, t101237: f64, t101240: f64, t25147: f64, t25150: f64, t25162: f64, t25164: f64, t28154: f64, t7702: f64, t7716: f64, t92570: f64, t92573: f64, t92577: f64, t92585: f64, t92690: f64) -> f64 {
    let t101243 = t2247 * t4187;
    let t101252 = t10309 * t1470;
    let t101259 = -10.0_f64 / 3.0_f64 * t25162 * t101227 - 10.0_f64 / 3.0_f64 * t101230 * t25164 + 35.0_f64 * t92690 * t101234 - 10.0_f64 / 3.0_f64 * t101237 * t25164 - 10.0_f64 / 3.0_f64 * t101240 * t25164 - 10.0_f64 / 3.0_f64 * t101243 * t25164 - 10.0_f64 / 3.0_f64 * t28154 * t92573 - 10.0_f64 / 3.0_f64 * t28154 * t92577 - 5.0_f64 / 3.0_f64 * t28154 * t92585 + 10.0_f64 * t101252 * t92570 - t7702 * t25147 / 6.0_f64 - t25150 * t7716 / 6.0_f64;
    t101259
}
