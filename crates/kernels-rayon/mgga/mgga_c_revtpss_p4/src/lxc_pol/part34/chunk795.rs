//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 795/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk795(t14: f64, t588: f64, t521: f64, t2516: f64, t676: f64, t3869: f64, t2496: f64, t4010: f64, t73: f64, t1386: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = 144.0_f64 * t9856;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1_f64 * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = 0.48159733137676571078e0_f64 * t3869 * t9866;
    let t9880 = t73 * t4010;
    let t9909 = t820 * t1386 * t2681;
    (t9857, t9863, t9865, t9866, t9868, t9880, t9909)
}
