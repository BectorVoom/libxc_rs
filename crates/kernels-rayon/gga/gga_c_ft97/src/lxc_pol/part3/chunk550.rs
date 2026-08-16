//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 550/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk550(t1558: f64, t4417: f64, t1555: f64, t89: f64, t925: f64, t942: f64, t1564: f64, t446: f64, t1571: f64, t356: f64, t1578: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4418 = t1558 * t4417;
    let t4420 = t89 * t1555 * t4418;
    let t4422 = t925 * t942;
    let t4423 = t1564 * t4422;
    let t4424 = t446 * t4423;
    let t4426 = t1571 * t4417;
    let t4428 = t89 * t356 * t4426;
    let t4431 = 2.0_f64 * t361 + 2.0_f64 * t1578;
    (t4418, t4420, t4422, t4423, t4424, t4426, t4428, t4431)
}
