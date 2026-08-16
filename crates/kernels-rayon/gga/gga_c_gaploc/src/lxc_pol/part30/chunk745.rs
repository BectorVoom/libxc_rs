//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 745/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk745(t7029: f64, t7030: f64, t4786: f64, t586: f64, t1323: f64, t161: f64, t165: f64, t912: f64, t2488: f64, t6895: f64, t2487: f64, t1392: f64, t2344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7031 = t7029 * t7030;
    let t7033 = t4786 * t586;
    let t7035 = t161 * t165 * t1323;
    let t7036 = t912 * t7035;
    let t7037 = t7033 * t7036;
    let t7039 = t2488 * t6895;
    let t7040 = t2487 * t7039;
    let t7042 = t1392 * t2344;
    (t7031, t7033, t7035, t7037, t7040, t7042)
}
