//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1460/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1460(t3117: f64, t4571: f64, t248: f64, t3051: f64, t4347: f64, t1041: f64, t3114: f64, t4630: f64, t3101: f64, t4650: f64, t1020: f64, t10508: f64, t1616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13948 = t3117 * t4571 / 3456.0_f64;
    let t13950 = t248 * t3051 * t4347;
    let t13952 = t1041 * t13950 / 3456.0_f64;
    let t13959 = t3114 * t4630 / 2304.0_f64;
    let t13961 = t248 * t3101 * t4650;
    let t13963 = t1020 * t13961 / 2304.0_f64;
    let t13965 = t248 * t10508 * t1616;
    (t13948, t13950, t13952, t13959, t13961, t13963, t13965)
}
