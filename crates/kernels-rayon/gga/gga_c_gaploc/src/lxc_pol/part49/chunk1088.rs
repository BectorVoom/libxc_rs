//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1088/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1088(t38392: f64, t874: f64, t2268: f64, t2343: f64, t1358: f64, t13777: f64, t2299: f64, t488: f64, t1365: f64, t38272: f64, t6525: f64, t426: f64, t46849: f64, t535: f64) -> (f64, f64, f64, f64, f64) {
    let t47026 = t38392 * t874;
    let t47028 = t2268 * t2343 * t47026;
    let t47032 = t1358 * t2299 * t13777 * t488;
    let t47036 = t6525 * t1365 * t38272;
    let t47040 = t2268 * t535 * t46849 * t426;
    (t47026, t47028, t47032, t47036, t47040)
}
