//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 919/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk919(t1983: f64, t7585: f64, t7586: f64, t930: f64, t7832: f64, t7839: f64, t1098: f64, t7614: f64, t1108: f64, t7746: f64, t1086: f64, t1113: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30967 = t7585 * t7586 * t1983 * t930;
    let t30974 = t7839 * t7832;
    let t30976 = t7614 * t1098;
    let t30978 = t7746 * t1108;
    let t30980 = t7614 * t1086;
    let t30982 = t7746 * t1113;
    (t30967, t30974, t30976, t30978, t30980, t30982)
}
