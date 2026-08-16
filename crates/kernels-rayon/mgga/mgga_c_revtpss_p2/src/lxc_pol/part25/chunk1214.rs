//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1214/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1214(t10406: f64, t76: f64, t38: f64, t45955: f64, t2242: f64, t2251: f64, t2247: f64, t25138: f64, t1923: f64, t1926: f64, t1928: f64, t25102: f64, t25106: f64, t25110: f64, t25139: f64, t25143: f64, t25146: f64, t25147: f64, t25150: f64, t6954: f64, t6960: f64, t6973: f64, t6974: f64, t6977: f64, t6978: f64) -> f64 {
    let t92628 = t76 * t10406;
    let t92632 = t45955 * t38;
    let t92639 = t2242 * t2251;
    let t92644 = t2247 * t38 * t25138;
    let t92649 = -t1923 * t25139 * t6977 / 2.0_f64 - t6954 * t25143 - t1923 * t6973 * t25146 / 2.0_f64 - t6954 * t25147 / 2.0_f64 - t1923 * t1926 * t92628 / 6.0_f64 - t92632 * t1928 / 6.0_f64 - t25150 * t6974 / 2.0_f64 - t25150 * t6978 / 2.0_f64 + t92639 * t1928 + 2.0_f64 * t25102 * t6978 + 5.0_f64 / 2.0_f64 * t92644 * t6960 + 5.0_f64 * t25106 * t25110;
    t92649
}
