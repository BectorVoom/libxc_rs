//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 902/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk902(t3051: f64, t471: f64, t458: f64, t8272: f64, t2: f64, t32075: f64, t1771: f64, t1806: f64, t1802: f64, t11176: f64, t94: f64, t432: f64, t8376: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38504 = t3051 * t471;
    let t38506 = t458 * t8272;
    let t38508 = t32075 * t2;
    let t38513 = t1771 * t1806;
    let t38519 = t1771 * t1802;
    let t38525 = 280.0_f64 / 81.0_f64 * t11176 * t94;
    let t38526 = t8376 * t432;
    (t38504, t38506, t38508, t38513, t38519, t38525, t38526)
}
