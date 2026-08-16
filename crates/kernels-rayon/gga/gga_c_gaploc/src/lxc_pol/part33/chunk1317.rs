//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1317/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1317(t34488: f64, t6895: f64, t9263: f64, t993: f64, t34371: f64, t6963: f64, t6964: f64, t10354: f64, t20003: f64, t1402: f64, t1429: f64, t3380: f64) -> (f64, f64, f64, f64, f64) {
    let t34489 = 0.76685851907841499352e0_f64 * t34488;
    let t34491 = t9263 * t993 * t6895;
    let t34492 = 0.38342925953920749676e0_f64 * t34491;
    let t34498 = 0.14300195980740170668e1_f64 * t6963 * t6964 * t34371;
    let t34500 = 0.23005755572352449806e2_f64 * t20003 * t10354;
    let t34502 = t1429 * t1402 * t3380;
    (t34489, t34492, t34498, t34500, t34502)
}
