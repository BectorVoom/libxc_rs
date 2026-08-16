//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 940/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk940(t1882: f64, t20413: f64, t20215: f64, t8392: f64, t20431: f64, t20403: f64, t8417: f64, t20421: f64, t20424: f64, t20248: f64, t20284: f64, t20188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74745 = t1882 * t20413;
    let t74755 = t8392 * t20215;
    let t74757 = t8392 * t20431;
    let t74759 = t8417 * t20403;
    let t74778 = t1882 * t20421;
    let t74780 = t1882 * t20424;
    let t74786 = t1882 * t20248;
    let t74809 = t1882 * t20284;
    let t74861 = t1882 * t20188;
    (t74745, t74755, t74757, t74759, t74778, t74780, t74786, t74809, t74861)
}
