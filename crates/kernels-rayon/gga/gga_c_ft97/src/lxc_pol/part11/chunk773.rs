//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 773/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk773(t10465: f64, t2882: f64, t2881: f64, t4265: f64, t9853: f64, t4140: f64, t4139: f64, t2344: f64, t798: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10466 = t2882 * t10465;
    let t10467 = t2881 * t10466;
    let t10470 = t4265 * t9853;
    let t10471 = t2881 * t10470;
    let t10474 = t4140 * t9853;
    let t10475 = t4139 * t10474;
    let t10478 = t2344 * t798;
    (t10466, t10467, t10470, t10471, t10474, t10475, t10478)
}
