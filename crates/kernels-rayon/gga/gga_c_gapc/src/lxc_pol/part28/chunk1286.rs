//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1286/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1286(t11594: f64, t21838: f64, t21631: f64, t11397: f64, t11402: f64, t424: f64, t11401: f64, t3074: f64, t35085: f64, t27036: f64, t27043: f64, t35139: f64) -> (f64, f64, f64, f64, f64) {
    let t35341 = t11594 * t21838;
    let t35343 = t11594 * t21631;
    let t35346 = t424 * t11397 * t11402;
    let t35348 = t3074 * t11401;
    let t35349 = t35085 * t35348;
    let t35352 = t27036 * t35139 * t27043;
    (t35341, t35343, t35346, t35349, t35352)
}
