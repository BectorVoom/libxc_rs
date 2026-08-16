//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 578/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk578(t213: f64, t709: f64, t231: f64, t6819: f64, t17836: f64, t3773: f64, t1113: f64, t679: f64, t689: f64) -> (f64, f64, f64) {
    let t27522 = t213 * t709;
    let t27523 = t231 * t27522;
    let t27524 = t6819 * t27523;
    let t27527 = t17836 * t3773;
    let t27528 = t1113 * t679;
    let t27529 = t27528 * t689;
    (t27524, t27527, t27529)
}
