//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1285/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1285(t1386: f64, t820: f64, t844: f64, t2482: f64, t596: f64, t4021: f64, t1384: f64, t235: f64, t4003: f64, t543: f64, t27: f64, t4000: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9962 = t820 * t1386 * t844;
    let t9976 = t2482 * t1386 * t596;
    let t9977 = t9976 * t4021;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0_f64 / t9989;
    let t9991 = t9990 * t235;
    let t9994 = t4003 * t543;
    let t10001 = t2482 * t4000 * t27;
    (t9962, t9976, t9977, t9990, t9991, t9994, t10001)
}
