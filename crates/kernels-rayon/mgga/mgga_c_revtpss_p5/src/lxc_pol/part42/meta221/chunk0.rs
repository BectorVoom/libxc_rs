//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 859/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk859(t2339: f64, t5891: f64, t1504: f64, t2349: f64, t100: f64, t5823: f64, t1479: f64, t1509: f64, t2357: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5892 = t2339 * t5891;
    let t5895 = t1504 * t1504;
    let t5896 = t2349 * t5895;
    let t5899 = t100 * t5823;
    let t5902 = tau1 * t1479;
    let t5907 = t1509 * t1509;
    let t5908 = t2357 * t5907;
    let t5911 = -t5823;
    (t5892, t5895, t5896, t5899, t5902, t5907, t5908, t5911)
}
