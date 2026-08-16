//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 681/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk681(t145: f64, t459: f64, t6361: f64, t1232: f64, t1236: f64, t1242: f64, t1233: f64, t130: f64, t1234: f64, t137: f64, t453: f64, t4074: f64, t4077: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6363 = t6361 * t145 * t459;
    let t6365 = t1232 * t1236;
    let t6366 = t6365 * t1242;
    let t6368 = t130 * t1233;
    let t6371 = 1.0_f64 / t137 / t1234 / t453;
    let t6372 = t6368 * t6371;
    let t6374 = t6372 * t4074 * t4077;
    (t6363, t6365, t6366, t6371, t6372, t6374)
}
