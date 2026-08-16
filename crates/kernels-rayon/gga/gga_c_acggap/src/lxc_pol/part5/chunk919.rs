//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 919/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk919(t14047: f64, t3367: f64, t3378: f64, t3402: f64, t3077: f64, t3371: f64, t1167: f64, t10098: f64, t1172: f64) -> (f64, f64, f64, f64, f64) {
    let t14048 = t14047 * t3367;
    let t14050 = t3378 * t3402;
    let t14053 = t3077 * t3371;
    let t14054 = t14053 * t1167;
    let t14056 = t10098 * t1172;
    (t14048, t14050, t14053, t14054, t14056)
}
