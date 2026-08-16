//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 690/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk690(t6962: f64, t92: f64, t1253: f64, t6261: f64, t193: f64, t4299: f64, t6353: f64, t1476: f64, t2665: f64, t684: f64, t2749: f64, t7124: f64) -> (f64, f64, f64, f64, f64) {
    let t29008 = t6962 * t92;
    let t29016 = t6261 * t1253;
    let t29017 = t193 * t29016;
    let t29020 = t6353 * t4299;
    let t29024 = t1476 * t1253;
    let t29026 = t2665 * t29024 * t684;
    let t29030 = t2749 * t7124;
    (t29008, t29017, t29020, t29026, t29030)
}
