//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 314/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk314(t123: f64, t1359: f64, t488: f64, t197: f64, t160: f64) -> (f64, f64, f64, f64) {
    let t1360 = t1359 * t123;
    let t1361 = t1360 * t488;
    let t1364 = t197 * t123;
    let t1365 = t1364 * t160;
    (t1360, t1361, t1364, t1365)
}
