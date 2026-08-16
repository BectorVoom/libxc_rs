//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 960/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk960(t43386: f64, t33289: f64, t9800: f64, t9806: f64, t43007: f64, t5241: f64, t5640: f64, t590: f64, t43107: f64, t11068: f64, t2679: f64, t9796: f64) -> (f64, f64, f64, f64, f64) {
    let t43387 = 0.76685851907841499353e0_f64 * t43386;
    let t43389 = t9800 * t33289 * t9806;
    let t43390 = 0.72851559312449424385e1_f64 * t43389;
    let t43393 = t5640 * t5241 * t43007 * t590;
    let t43398 = 0.15337170381568299871e1_f64 * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    (t43387, t43390, t43393, t43398, t43400)
}
