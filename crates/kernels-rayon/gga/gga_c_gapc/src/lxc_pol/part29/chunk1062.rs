//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1062/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1062(t11913: f64, t28427: f64, t435: f64, t9281: f64, t1084: f64, t3415: f64, t11784: f64, t9865: f64, t11379: f64, t11945: f64, t28594: f64, t11948: f64, t30095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33156 = t11913 * t28427;
    let t33158 = t435 * t9281;
    let t33160 = t1084 * t33158 * t3415;
    let t33162 = t11784 * t9865;
    let t33165 = t28594 * t11379 * t11945;
    let t33167 = t11948 * t30095;
    (t33156, t33158, t33160, t33162, t33165, t33167)
}
