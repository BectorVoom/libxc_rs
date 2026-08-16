//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 799/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk799(t3132: f64, t5395: f64, t5392: f64, t3128: f64, t5633: f64, t3133: f64, t633: f64, t8992: f64, t1835: f64, t1691: f64, t129: f64, t4948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9336 = t5395 * t3132;
    let t9337 = t9336 * t5392;
    let t9339 = t3128 * t5633;
    let t9341 = t3133 * t5633;
    let t9343 = t633 * t8992;
    let t9344 = t9343 * t1835;
    let t9346 = t9343 * t1691;
    let t9348 = t4948 * t129;
    (t9337, t9339, t9341, t9344, t9346, t9348)
}
