//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 202/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk202(t1384: f64, t605: f64, t144: f64, t1366: f64, t1380: f64, t28: f64, t446: f64, t89: f64) -> (f64, f64) {
    let t1385 = t605 * t1384;
    let t1386 = t144 * t1385;
    let t1389 = t89 * t28 * t1380 / 3.0_f64 - t446 * t1366 / 3.0_f64 - t446 * t1386 / 3.0_f64;
    (t1386, t1389)
}
