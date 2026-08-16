//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 538/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk538(t5883: f64, t94: f64, t1518: f64, t1843: f64, t1513: f64, t2339: f64, t1504: f64, t2349: f64, t100: f64, t5823: f64, t1479: f64, t1509: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5884 = t94 * t5883;
    let t5887 = t1843 * t1518;
    let t5891 = t1513 * t1513;
    let t5892 = t2339 * t5891;
    let t5895 = t1504 * t1504;
    let t5896 = t2349 * t5895;
    let t5899 = t100 * t5823;
    let t5902 = tau1 * t1479;
    let t5907 = t1509 * t1509;
    (t5884, t5887, t5891, t5892, t5895, t5896, t5899, t5902, t5907)
}
