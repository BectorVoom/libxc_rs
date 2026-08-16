//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1056/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1056(t11279: f64, t575: f64, t687: f64, t3721: f64, t4905: f64, t8601: f64, t8616: f64, t3179: f64, t8598: f64, t11706: f64, t883: f64, t2468: f64, t3742: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33111 = t11279 * t575;
    let t33113 = 2.0_f64 * t33111 * t687;
    let t33114 = t4905 * t3721;
    let t33116 = 4.0_f64 * t8601 * t8616;
    let t33119 = 4.0_f64 * t8598 * t3179;
    let t33121 = t11706 * t883;
    let t33129 = t3742 * t2468;
    (t33113, t33114, t33116, t33119, t33121, t33129)
}
