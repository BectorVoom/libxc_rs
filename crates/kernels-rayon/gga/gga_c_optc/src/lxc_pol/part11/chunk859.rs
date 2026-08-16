//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 859/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk859(t3593: f64, t4599: f64, t1256: f64, t13509: f64, t13526: f64, t16287: f64, t16337: f64, t16339: f64, t16340: f64, t1879: f64, t3539: f64, t4595: f64, t606: f64, t6477: f64, t6811: f64, t95: f64, t9535: f64) -> f64 {
    let t16595 = t3593 * t4599;
    let t16602 = t16337 + 3.0_f64 / 2.0_f64 * t9535 + t16339 + 0.23260393291413087447e-1_f64 * t1879 * t3593 * t4595 + 0.77534644304710291488e-2_f64 * t95 * t606 * t16287 + 0.46520786582826174894e-1_f64 * t3539 * t16595 + t6811 + 3.0_f64 / 2.0_f64 * t13526 + t6477 - 0.23260393291413087447e-1_f64 * t1879 * t13509 * t1256 + t16340;
    t16602
}
