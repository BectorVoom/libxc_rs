//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 861/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk861(t1554: f64, t1586: f64, t2: f64, t355: f64, t7241: f64, t369: f64, t7760: f64, t32075: f64, t11176: f64, t94: f64, t37406: f64, t37352: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38463 = t1554 * t1586;
    let t38464 = t38463 * t2;
    let t38477 = t355 * t7241;
    let t38478 = t38477 * t2;
    let t38482 = t7760 * t369;
    let t38483 = t38482 * t2;
    let t38508 = t32075 * t2;
    let t38525 = 280.0_f64 / 81.0_f64 * t11176 * t94;
    let t38549 = t2 * t37406;
    let t38570 = t37352 * t82;
    (t38463, t38464, t38477, t38478, t38482, t38483, t38508, t38525, t38549, t38570)
}
