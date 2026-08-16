//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1161/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1161(t10296: f64, t10288: f64, t10286: f64, t10285: f64, t10290: f64, t10298: f64, t4349: f64, t605: f64, t1651: f64, t3366: f64, t27214: f64, t6565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31447 = 12.0_f64 * t10296;
    let t31448 = 2.0_f64 * t10288;
    let t31449 = 4.0_f64 * t10286;
    let t31453 = 2.0_f64 * t10285;
    let t31454 = 4.0_f64 * t10290;
    let t31458 = 12.0_f64 * t4349 * t10298 * t605;
    let t31461 = 6.0_f64 * t4349 * t3366 * t1651;
    let t31463 = 6.0_f64 * t27214 * t6565;
    (t31447, t31448, t31449, t31453, t31454, t31458, t31461, t31463)
}
