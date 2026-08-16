//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1157/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1157(t1923: f64, t2047: f64, t2048: f64, t25117: f64, t25150: f64, t26172: f64, t6954: f64, t7352: f64, t92628: f64, t92632: f64, t95230: f64, t95241: f64, t95243: f64, t95246: f64, t95248: f64, t95253: f64) -> f64 {
    let t95254 = -8.0_f64 / 3.0_f64 * t95230 + t1923 * t2047 * t92628 / 3.0_f64 - 2.0_f64 * t25117 * t7352 + t92632 * t2048 / 3.0_f64 + t25150 * t7352 + t6954 * t26172 - 8.0_f64 / 3.0_f64 * t95241 - 16.0_f64 / 3.0_f64 * t95243 + 88.0_f64 / 9.0_f64 * t95246 + 16.0_f64 / 3.0_f64 * t95248 - t95253;
    t95254
}
