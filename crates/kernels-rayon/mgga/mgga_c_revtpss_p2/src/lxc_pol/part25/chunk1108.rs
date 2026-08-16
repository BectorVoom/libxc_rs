//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1108/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1108(t2248: f64, t77: f64, t84: f64, t2247: f64, t607: f64, t1927: f64, t644: f64, t1926: f64, t1923: f64, t1928: f64, t25099: f64, t25102: f64, t25106: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t25140: f64, t25143: f64, t25147: f64, t25150: f64, t25157: f64, t6954: f64, t6958: f64, t6960: f64, t6963: f64, t6974: f64, t6978: f64) -> (f64, f64, f64, f64, f64) {
    let t25159 = t77 * t84 * t2248;
    let t25162 = t2247 * t607;
    let t25163 = t1927 * t644;
    let t25164 = t1926 * t25163;
    let t25167 = 5.0_f64 / 3.0_f64 * t25099 * t6960 + 2.0_f64 / 3.0_f64 * t25102 * t1928 + 5.0_f64 / 3.0_f64 * t25106 * t6960 + 5.0_f64 / 3.0_f64 * t6958 * t25110 + 5.0_f64 / 6.0_f64 * t6958 * t25114 + t25117 * t1928 / 3.0_f64 + t25120 * t1928 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6963 * t6974 + 2.0_f64 / 3.0_f64 * t6963 * t6978 - t1923 * t25140 / 6.0_f64 - t1923 * t25143 / 3.0_f64 - t1923 * t25147 / 6.0_f64 - t25150 * t1928 / 6.0_f64 - t6954 * t6974 / 3.0_f64 - t6954 * t6978 / 3.0_f64 - 5.0_f64 * t25157 * t25159 - 10.0_f64 / 3.0_f64 * t25162 * t25164;
    (t25159, t25162, t25163, t25164, t25167)
}
