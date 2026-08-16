//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 892/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk892(t3091: f64, t455: f64, t145: f64, t459: f64, t129: f64, t2276: f64, t1242: f64, t1232: f64, t130: f64, t1234: f64, t136: f64, t137: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9095 = t3091 * t455;
    let t9097 = t9095 * t145 * t459;
    let t9099 = t129 * t2276;
    let t9100 = t9099 * t1242;
    let t9102 = t130 * t1232;
    let t9103 = t1234 * t136;
    let t9105 = 1.0_f64 / t137 / t9103;
    (t9095, t9097, t9099, t9100, t9102, t9105)
}
