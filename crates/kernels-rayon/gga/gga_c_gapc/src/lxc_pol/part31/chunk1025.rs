//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1025/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1025(t11784: f64, t3330: f64, t10058: f64, t3784: f64, t11311: f64, t916: f64, t1086: f64, t6188: f64, t11320: f64, t2636: f64, t9554: f64, t129: f64, t7451: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11785 = t11784 * t3330;
    let t11787 = t3784 * t10058;
    let t11790 = t916 * t11311;
    let t11791 = t1086 * t6188;
    let t11792 = t11790 * t11791;
    let t11794 = t916 * t11320;
    let t11795 = t2636 * t9554;
    let t11796 = t11794 * t11795;
    let t11798 = t7451 * t129;
    (t11785, t11787, t11790, t11791, t11792, t11794, t11795, t11796, t11798)
}
