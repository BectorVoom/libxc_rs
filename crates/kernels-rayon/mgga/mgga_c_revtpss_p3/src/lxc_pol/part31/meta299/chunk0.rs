//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1288/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1288(t10073: f64, t4089: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64, t136: f64, t1419: f64, t2457: f64, t3964: f64, t225: f64, t9646: f64) -> (f64, f64, f64, f64, f64) {
    let t10074 = t10073 * t4089;
    let t10098 = t1432 * t4107 * t2470;
    let t10102 = 0.30356481678079769392e-1_f64 * t1432 * t1433 * t9288;
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    (t10074, t10098, t10102, t10109, t10111)
}
