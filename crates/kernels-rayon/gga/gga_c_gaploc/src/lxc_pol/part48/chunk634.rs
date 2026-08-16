//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 634/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk634(t3565: f64, t528: f64, t1645: f64, t2792: f64, t3556: f64, t1: f64, t3516: f64, t106: f64, t192: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11389 = t528 * t3565;
    let t11392 = t1645 * t2792;
    let t11395 = t528 * t3556;
    let t11400 = t3516 * t1;
    let t11401 = t11400 * t106;
    let t11402 = t11401 * t192;
    (t11389, t11392, t11395, t11400, t11401, t11402)
}
