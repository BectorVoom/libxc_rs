//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1265/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1265(t1026: f64, t1046: f64, t1266: f64, t11424: f64, t3665: f64, t561: f64, t5325: f64, t5977: f64, t5979: f64, t116: f64, t33965: f64, t11402: f64) -> (f64, f64, f64, f64, f64) {
    let t35077 = t1266 * t1026 * t1046;
    let t35080 = t561 * t11424 * t3665;
    let t35083 = t5977 * t5325 * t5979;
    let t35085 = t116 * t33965;
    let t35086 = t35085 * t11402;
    (t35077, t35080, t35083, t35085, t35086)
}
