//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 426/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk426(t1397: f64, t1428: f64, t2366: f64, t486: f64, t1423: f64, t1: f64, t594: f64, t106: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4379 = t1397 * t1428;
    let t4385 = t486 * t2366;
    let t4386 = t1423 * t4385;
    let t4389 = t594 * t1;
    let t4390 = t4389 * t106;
    let t4391 = t544 * t4390;
    (t4379, t4385, t4386, t4389, t4390, t4391)
}
