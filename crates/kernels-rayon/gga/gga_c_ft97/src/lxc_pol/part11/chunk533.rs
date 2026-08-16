//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 533/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk533(t408: f64, t6: f64, t1693: f64, t1710: f64, t139: f64, t1995: f64, t527: f64, t135: f64, t542: f64, t1711: f64, t39: f64, t64: f64, rho0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5566 = t408 * t6;
    let t5588 = t1693 * rho0;
    let t5596 = t1710 * t6;
    let t5784 = t139 * t6;
    let t5785 = t1995 * t5784;
    let t5802 = t527 * t5784;
    let t5818 = t542 * t135;
    let t7201 = t1711 * t39;
    let t7202 = t64 * t7201;
    (t5566, t5588, t5596, t5785, t5802, t5818, t7202)
}
