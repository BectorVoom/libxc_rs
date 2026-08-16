//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 927/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk927(t11465: f64, t23451: f64, t3014: f64, t981: f64, t3011: f64, t973: f64, t1610: f64, t19056: f64, t4590: f64, t6142: f64, t15421: f64, t6145: f64) -> (f64, f64, f64, f64, f64) {
    let t23452 = t11465 * t23451;
    let t23453 = t23452 * t3014;
    let t23455 = 0.10389515463408878255e3_f64 * t981 * t23453;
    let t23457 = t3011 * t23451 * t973;
    let t23459 = 0.35089341735807877242e1_f64 * t981 * t23457;
    let t23461 = 3.0_f64 * t19056 * t1610;
    let t23463 = 3.0_f64 * t4590 * t6142;
    let t23465 = 0.48245938496077605201e2_f64 * t15421 * t6145;
    (t23455, t23459, t23461, t23463, t23465)
}
