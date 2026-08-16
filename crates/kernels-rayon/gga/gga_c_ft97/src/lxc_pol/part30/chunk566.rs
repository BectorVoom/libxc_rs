//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 566/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk566(t1472: f64, t24287: f64, t1476: f64, t2360: f64, t1486: f64, t6327: f64, t681: f64, t1491: f64, t1636: f64, t89: f64, t1485: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25132 = 0.11113000182098765433e-1_f64 * t1472 * t24287;
    let t25140 = t1476 * t2360;
    let t25146 = t1486 * t681 * t6327;
    let t25153 = t89 * t1636 * t1491;
    let t25154 = 4.0_f64 / 9.0_f64 * t25153;
    let t25162 = t1485 * t458;
    (t25132, t25140, t25146, t25153, t25154, t25162)
}
