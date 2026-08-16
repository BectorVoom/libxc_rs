//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1216/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1216(t10327: f64, t603: f64, t1928: f64, t25106: f64, t25114: f64, t25120: f64, t25140: f64, t25143: f64, t6958: f64, t6960: f64, t6963: f64, t6974: f64, t6978: f64, t92654: f64, t92658: f64, t92662: f64, t92666: f64, t92669: f64, t92672: f64) -> f64 {
    let t92674 = t603 * t10327;
    let t92682 = 5.0_f64 / 2.0_f64 * t25106 * t25114 + 5.0_f64 / 2.0_f64 * t6958 * t92654 + 5.0_f64 / 2.0_f64 * t6958 * t92658 + 5.0_f64 / 6.0_f64 * t6958 * t92662 - 5.0_f64 * t92666 * t6960 + t603 * t92669 * t92672 + t92674 * t1928 / 3.0_f64 + t25120 * t6974 + t25120 * t6978 + t6963 * t25140 + 2.0_f64 * t6963 * t25143;
    t92682
}
