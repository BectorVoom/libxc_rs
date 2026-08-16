//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 968/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk968(t11311: f64, t916: f64, t1086: f64, t6188: f64, t11320: f64, t2636: f64, t9554: f64, t129: f64, t7451: f64, t3284: f64, t7453: f64, t190: f64, t277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11790 = t916 * t11311;
    let t11791 = t1086 * t6188;
    let t11792 = t11790 * t11791;
    let t11794 = t916 * t11320;
    let t11795 = t2636 * t9554;
    let t11796 = t11794 * t11795;
    let t11798 = t7451 * t129;
    let t11799 = t3284 * t7453;
    let t11800 = t11798 * t11799;
    let t11802 = t277 * t190;
    (t11790, t11791, t11792, t11794, t11795, t11796, t11798, t11799, t11800, t11802)
}
