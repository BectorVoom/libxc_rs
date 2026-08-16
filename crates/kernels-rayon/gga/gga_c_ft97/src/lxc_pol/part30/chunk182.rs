//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 182/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk182(t1410: f64, t237: f64, t1100: f64, t6: f64, t695: f64, t224: f64) -> (f64, f64, f64, f64) {
    let t1411 = t237 * t1410;
    let t1412 = t1100 * t1411;
    let t1416 = t695 * t6;
    let t1417 = t224 * t1416;
    (t1411, t1412, t1416, t1417)
}
