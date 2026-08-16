//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 632/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk632(t1535: f64, t4416: f64, t584: f64, t585: f64, t1406: f64, t1435: f64, t121: f64, t1508: f64, t1397: f64, t1420: f64, t1: f64, t4149: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4417 = t1535 * t4416;
    let t4418 = t584 * t4417;
    let t4421 = t585 * t4416;
    let t4425 = t584 * t4421;
    let t4428 = t1406 * t1435;
    let t4461 = t1508 * t121;
    let t4494 = t1397 * t1420;
    let t4501 = t4149 * t1;
    (t4418, t4425, t4428, t4461, t4494, t4501)
}
