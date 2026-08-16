//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1975/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1975(t1497: f64, t640: f64, t77: f64, t4241: f64, t84: f64, t1470: f64, t2242: f64, t1923: f64, t1928: f64, t25106: f64, t28078: f64, t28081: f64, t28086: f64, t28090: f64, t28093: f64, t6954: f64, t6958: f64, t6974: f64, t6978: f64, t7702: f64, t7706: f64, t7716: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    let t28109 = t77 * t28108;
    let t28112 = t2242 * t1470;
    let t28115 = -t1923 * t28078 / 6.0_f64 - t1923 * t28081 / 6.0_f64 - t6954 * t7720 / 6.0_f64 - t1923 * t28086 / 6.0_f64 - t1923 * t28090 / 6.0_f64 - t28093 * t1928 / 6.0_f64 - t7702 * t6974 / 6.0_f64 - t7702 * t6978 / 6.0_f64 - t6954 * t7716 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t25106 * t7706 + 5.0_f64 / 6.0_f64 * t6958 * t28105 + 5.0_f64 / 6.0_f64 * t6958 * t28109 + t28112 * t1928 / 3.0_f64;
    (t28104, t28105, t28108, t28109, t28112, t28115)
}
