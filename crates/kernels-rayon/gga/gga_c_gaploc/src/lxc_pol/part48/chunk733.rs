//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 733/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk733(t160: f64, t19531: f64, t4348: f64, t892: f64, t10523: f64, t1422: f64, t544: f64, t1564: f64, t165: f64, t10524: f64, t1415: f64, t1433: f64, t9271: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19532 = t19531 * t160;
    let t19933 = t892 * t4348;
    let t20367 = t544 * t10523 * t1422;
    let t20368 = t165 * t1564;
    let t20471 = t1415 * t10524;
    let t20535 = t1433 * t9271;
    (t19532, t19933, t20367, t20368, t20471, t20535)
}
