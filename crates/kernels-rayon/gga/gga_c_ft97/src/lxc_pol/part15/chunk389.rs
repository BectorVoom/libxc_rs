//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 389/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk389(t2400: f64, t1546: f64, t89: f64, t921: f64, t1557: f64, t920: f64) -> (f64, f64, f64) {
    let t2946 = 0.19257444444444444444e0_f64 * t2400;
    let t2981 = t89 * t1546 * t921;
    let t2983 = t1557 * t920;
    (t2946, t2981, t2983)
}
