//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1157/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1157(t20900: f64, t7030: f64, t20374: f64, t7035: f64, t888: f64, t10296: f64, t10288: f64, t10286: f64, t10284: f64, t10282: f64, t10306: f64, t10285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31414 = 0.59584149919750711116e-1_f64 * t20900 * t7030;
    let t31416 = t20374 * t888 * t7035;
    let t31417 = 0.76685851907841499352e0_f64 * t31416;
    let t31447 = 12.0_f64 * t10296;
    let t31448 = 2.0_f64 * t10288;
    let t31449 = 4.0_f64 * t10286;
    let t31450 = 2.0_f64 * t10284;
    let t31451 = 2.0_f64 * t10282;
    let t31452 = 4.0_f64 * t10306;
    let t31453 = 2.0_f64 * t10285;
    (t31414, t31417, t31447, t31448, t31449, t31450, t31451, t31452, t31453)
}
