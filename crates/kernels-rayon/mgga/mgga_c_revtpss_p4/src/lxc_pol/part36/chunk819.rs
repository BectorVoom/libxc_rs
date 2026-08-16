//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 819/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk819(t548: f64, t9951: f64, t4010: f64, t72: f64, t245: f64, t1386: f64, t820: f64, t844: f64, t2482: f64, t596: f64, t1384: f64, t235: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9953 = 0.37792653007779990369e-1_f64 * t548 * t9951;
    let t9954 = t4010 * t72;
    let t9955 = t9954 * t245;
    let t9962 = t820 * t1386 * t844;
    let t9976 = t2482 * t1386 * t596;
    let t9989 = t1384 * t1384;
    let t9990 = 1.0_f64 / t9989;
    let t9991 = t9990 * t235;
    (t9953, t9955, t9962, t9976, t9990, t9991)
}
