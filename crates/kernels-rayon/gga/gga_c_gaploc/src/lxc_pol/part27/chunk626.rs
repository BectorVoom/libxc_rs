//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 626/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk626(t1305: f64, t172: f64, t1265: f64, t158: f64, t475: f64, t599: f64, t1328: f64, t1323: f64, t203: f64, t123: f64, t594: f64, t160: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4144 = t172 * t1305;
    let t4149 = t158 * t1265;
    let t4167 = t599 * t475;
    let t4183 = t599 * t1328;
    let t4245 = t172 * t1323;
    let t4250 = t158 * t1328;
    let t4255 = t203 * t1328;
    let t4260 = t594 * t123;
    let t4261 = t4260 * t160;
    (t4144, t4149, t4167, t4183, t4245, t4250, t4255, t4260, t4261)
}
