//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 939/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk939(t4719: f64, t6219: f64, t15101: f64, t6110: f64, t23466: f64, t935: f64, t2924: f64, t19467: f64, t4711: f64, t981: f64, t1699: f64, t6400: f64) -> (f64, f64, f64, f64, f64) {
    let t23562 = 0.35089341735807877242e1_f64 * t4719 * t6219;
    let t23564 = 6.0_f64 * t15101 * t6110;
    let t23565 = t23466 * t935;
    let t23567 = 6.0_f64 * t2924 * t23565;
    let t23568 = t19467 * t4711;
    let t23570 = 0.51947577317044391277e2_f64 * t981 * t23568;
    let t23571 = t6400 * t1699;
    (t23562, t23564, t23567, t23570, t23571)
}
