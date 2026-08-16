//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2257/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2257(t60224: f64, t6957: f64, t1493: f64, t2315: f64, t77: f64, t2259: f64, t4173: f64, t38: f64, t60248: f64, t1928: f64, t25114: f64, t25120: f64, t25140: f64, t25143: f64, t25159: f64, t28093: f64, t28127: f64, t28138: f64, t6958: f64, t6974: f64, t6978: f64, t7702: f64, t7716: f64, t7720: f64) -> f64 {
    let t101342 = t60224 * t6957;
    let t101350 = t77 * t1493 * t2315;
    let t101357 = t4173 * t2259;
    let t101360 = t60248 * t38;
    let t101371 = -5.0_f64 * t101342 * t25159 + 5.0_f64 / 6.0_f64 * t28127 * t25114 + t25120 * t7716 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t6958 * t101350 + t25120 * t7720 / 3.0_f64 + 5.0_f64 / 6.0_f64 * t28138 * t25114 + t101357 * t1928 / 3.0_f64 - t101360 * t1928 / 6.0_f64 - t28093 * t6974 / 3.0_f64 - t28093 * t6978 / 3.0_f64 - t7702 * t25140 / 6.0_f64 - t7702 * t25143 / 3.0_f64;
    t101371
}
