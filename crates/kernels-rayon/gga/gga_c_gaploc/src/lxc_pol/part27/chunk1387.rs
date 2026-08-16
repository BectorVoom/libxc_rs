//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1387/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1387(t12045: f64, t1641: f64, t34394: f64, t34397: f64, t34404: f64, t34406: f64, t34410: f64, t34414: f64, t34415: f64, t34416: f64, t34418: f64, t34420: f64, t34423: f64, t34425: f64, t34431: f64, t34435: f64) -> f64 {
    let t38541 = -0.92023022289409799224e1_f64 * t1641 * t12045 - t34394 - t34397 - t34404 - t34406 - t34410 - t34414 - t34415 + t34416 + t34418 - t34420 + t34423 + t34425 + t34431 + t34435;
    t38541
}
