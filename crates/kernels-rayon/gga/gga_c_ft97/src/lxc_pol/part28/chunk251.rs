//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 251/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk251(t118: f64, t29: f64, t341: f64, t343: f64, t123: f64, t532: f64, t129: f64, t39: f64, t11: f64, t1689: f64) -> (f64, f64, f64, f64, f64) {
    let t2007 = 1.0_f64 / t118 / t29;
    let t2014 = t341 * t343;
    let t2021 = t123 / t532 / t29;
    let t2034 = t129 * t39;
    let t2035 = t1689 * t11;
    (t2007, t2014, t2021, t2034, t2035)
}
