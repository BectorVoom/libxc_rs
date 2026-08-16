//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 239/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk239(t488: f64, t979: f64, t83: f64, t28: f64, t442: f64, t446: f64, t89: f64, t951: f64, t955: f64, t973: f64) -> (f64, f64, f64) {
    let t980 = t488 * t979;
    let t981 = t83 * t980;
    let t984 = -t442 - t446 * t951 / 9.0_f64 - t446 * t955 / 3.0_f64 + t89 * t28 * t973 / 3.0_f64 - t446 * t981 / 3.0_f64;
    (t980, t981, t984)
}
