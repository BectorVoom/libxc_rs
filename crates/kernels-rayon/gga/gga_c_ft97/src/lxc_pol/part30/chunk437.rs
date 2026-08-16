//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 437/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk437(t1486: f64, t193: f64, t7075: f64, t6334: f64, t992: f64, t2665: f64, t446: f64, t1212: f64, t6222: f64, t89: f64, t7021: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7077 = t1486 * t193 * t7075;
    let t7079 = t6334 * t992;
    let t7080 = t2665 * t7079;
    let t7081 = t446 * t7080;
    let t7083 = t6222 * t1212;
    let t7084 = t193 * t7083;
    let t7085 = t89 * t7084;
    let t7087 = t799 * t7021;
    (t7077, t7079, t7080, t7081, t7083, t7084, t7085, t7087)
}
