//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 292/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk292(t599: f64, t874: f64, t475: f64, t2343: f64, t2293: f64, t493: f64) -> (f64, f64, f64, f64) {
    let t2344 = t599 * t874;
    let t2345 = t2344 * t475;
    let t2346 = t2343 * t2345;
    let t2349 = t493 * t2293;
    (t2344, t2345, t2346, t2349)
}
