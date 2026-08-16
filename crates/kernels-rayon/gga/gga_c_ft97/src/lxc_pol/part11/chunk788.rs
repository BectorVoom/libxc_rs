//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 788/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk788(t10642: f64, t10659: f64, t295: f64, t312: f64, t2832: f64, t870: f64, t875: f64, t296: f64, t1882: f64, t2859: f64, t10510: f64, t10514: f64, t10518: f64, t10522: f64, t10526: f64, t10530: f64, t10533: f64, t10536: f64, t10539: f64, t10542: f64, t10545: f64, t10548: f64, t1901: f64, t193: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10660 = t10642 + t10659;
    let t10662 = t295 * t10660 * t312;
    let t10666 = t2832 * t870;
    let t10667 = t10666 * t875;
    let t10668 = t296 * t10667;
    let t10670 = t1882 * t2859;
    let t10672 = -2.0_f64 / 3.0_f64 * t1901 * t10510 + 4.0_f64 / 9.0_f64 * t10514 - 2.0_f64 * t446 * t10518 - t446 * t10522 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t446 * t10526 - t446 * t10530 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t10533 - t446 * t10536 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t10539 - 2.0_f64 / 3.0_f64 * t446 * t10542 - 2.0_f64 / 9.0_f64 * t10545 + 4.0_f64 / 9.0_f64 * t446 * t10548 + t89 * t193 * t10662 / 3.0_f64 - t446 * t10668 + 2.0_f64 / 27.0_f64 * t10670;
    (t10660, t10662, t10666, t10667, t10668, t10672)
}
