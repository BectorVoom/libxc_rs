//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 435/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk435(t7045: f64, t840: f64, t871: f64, t319: f64, t7021: f64, t1091: f64, t1508: f64, t835: f64, t1212: f64, t1234: f64, t1476: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7047 = t840 * t871 * t7045;
    let t7051 = t840 * t319 * t7021;
    let t7055 = t835 * t1508 * t1091;
    let t7059 = t840 * t1508 * t1212;
    let t7062 = t1476 * t1234;
    let t7063 = t852 * t7062;
    (t7047, t7051, t7055, t7059, t7062, t7063)
}
