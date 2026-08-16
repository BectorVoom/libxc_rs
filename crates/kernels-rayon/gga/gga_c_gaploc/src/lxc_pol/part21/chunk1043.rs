//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1043/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1043(t123: f64, t1559: f64, t160: f64, t4348: f64, t892: f64, t2486: f64, t4803: f64, t594: f64, t874: f64, t1265: f64, t1415: f64, t6953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19531 = t1559 * t123;
    let t19532 = t19531 * t160;
    let t19933 = t892 * t4348;
    let t20003 = t4803 * t2486;
    let t20008 = t594 * t874;
    let t20013 = t874 * t1265;
    let t20018 = t1415 * t6953;
    (t19532, t19933, t20003, t20008, t20013, t20018)
}
