//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 836/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk836(t1882: f64, t7826: f64, t369: f64, t7954: f64, t432: f64, t7955: f64, t446: f64, t1559: f64, t1580: f64, t7763: f64) -> (f64, f64, f64, f64) {
    let t37303 = t1882 * t7826;
    let t37305 = t7954 * t369;
    let t37306 = t7955 * t432;
    let t37308 = t446 * t37305 * t37306;
    let t37311 = t7763 * t1559 * t1580;
    (t37303, t37306, t37308, t37311)
}
