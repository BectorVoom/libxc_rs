//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1925/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1925(t29543: f64, t77: f64, t5872: f64, t84: f64, t5819: f64, t603: f64, t5826: f64, t5816: f64, t1923: f64, t1928: f64, t25157: f64, t28127: f64, t28138: f64, t28151: f64, t28154: f64, t29513: f64, t29526: f64, t29529: f64, t29533: f64, t29538: f64, t6958: f64, t7702: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29544 = t77 * t29543;
    let t29547 = t84 * t5872;
    let t29548 = t77 * t29547;
    let t29551 = t603 * t5819;
    let t29554 = t603 * t5826;
    let t29561 = t84 * t5816;
    let t29562 = t77 * t29561;
    let t29567 = -t29513 * t1928 / 6.0_f64 - t7702 * t7716 / 3.0_f64 - t7702 * t7720 / 3.0_f64 - t1923 * t29526 / 6.0_f64 - t1923 * t29529 / 3.0_f64 - t1923 * t29533 / 6.0_f64 + 5.0_f64 / 3.0_f64 * t28138 * t7706 + 2.0_f64 / 3.0_f64 * t29538 * t1928 + 5.0_f64 / 3.0_f64 * t28127 * t7706 + 5.0_f64 / 3.0_f64 * t6958 * t29544 + 5.0_f64 / 6.0_f64 * t6958 * t29548 + t29551 * t1928 / 3.0_f64 + t29554 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7709 * t7716 + 2.0_f64 / 3.0_f64 * t7709 * t7720 - 5.0_f64 * t25157 * t29562 - 10.0_f64 / 3.0_f64 * t28154 * t28151;
    (t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29567)
}
