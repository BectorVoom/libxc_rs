//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1310/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1310(t1470: f64, t21663: f64, t1497: f64, t5868: f64, t77: f64, t108772: f64, t108782: f64, t108995: f64, t1928: f64, t28127: f64, t28138: f64, t29526: f64, t29529: f64, t29533: f64, t29538: f64, t29544: f64, t6958: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64) -> f64 {
    let t114270 = t21663 * t1470;
    let t114288 = t77 * t5868 * t1497;
    let t114292 = 5.0_f64 / 2.0_f64 * t108995 * t7706 + t114270 * t1928 + 5.0_f64 * t108772 * t7706 + 2.0_f64 * t29538 * t7716 + 5.0_f64 * t28138 * t29544 + 2.0_f64 * t29538 * t7720 + 5.0_f64 / 2.0_f64 * t108782 * t7706 + t7709 * t29526 + 5.0_f64 * t28127 * t29544 + 2.0_f64 * t7709 * t29529 + 5.0_f64 / 2.0_f64 * t6958 * t114288 + t7709 * t29533;
    t114292
}
