//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1034/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1034(t123: f64, t2084: f64, t160: f64, t23: f64, t268: f64, t1933: f64, t62: f64, t1375: f64, t1381: f64, t4348: f64, t498: f64, t177: f64, t208: f64, t4347: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16879 = t2084 * t123;
    let t16880 = t16879 * t160;
    let t16889 = t23 * t268;
    let t16922 = t62 * t1933;
    let t17277 = t1375 * t1381;
    let t17288 = t498 * t4348;
    let t17293 = t177 / t4347 / t208;
    (t16880, t16889, t16922, t17277, t17288, t17293)
}
