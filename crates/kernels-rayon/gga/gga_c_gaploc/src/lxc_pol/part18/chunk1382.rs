//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1382/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1382(t34371: f64, t6963: f64, t6964: f64, t10354: f64, t20003: f64, t1402: f64, t1429: f64, t3380: f64, t1: f64, t106: f64, t4524: f64, t544: f64) -> (f64, f64, f64, f64) {
    let t34498 = 0.14300195980740170668e1_f64 * t6963 * t6964 * t34371;
    let t34500 = 0.23005755572352449806e2_f64 * t20003 * t10354;
    let t34502 = t1429 * t1402 * t3380;
    let t34503 = 0.89376224879626066674e-1_f64 * t34502;
    let t34506 = t544 * t4524 * t1 * t106;
    (t34498, t34500, t34503, t34506)
}
