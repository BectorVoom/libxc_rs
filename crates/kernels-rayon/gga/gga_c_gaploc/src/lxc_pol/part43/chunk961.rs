//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 961/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk961(t13870: f64, t1890: f64, t1966: f64, t590: f64, t12240: f64, t2679: f64, t9800: f64, t3720: f64, t5241: f64, t9805: f64, t1991: f64, t47130: f64, t739: f64) -> (f64, f64, f64, f64) {
    let t47164 = 0.51123901271894332902e0_f64 * t1966 * t1890 * t13870 * t590;
    let t47166 = t9800 * t12240 * t2679;
    let t47168 = t5241 * t3720;
    let t47170 = t9805 * t47168 * t2679;
    let t47174 = t1991 * t739 * t47130 * t590;
    (t47164, t47166, t47170, t47174)
}
