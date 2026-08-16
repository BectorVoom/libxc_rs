//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 905/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk905(t33289: f64, t9800: f64, t9806: f64, t43107: f64, t5241: f64, t5640: f64, t590: f64, t11068: f64, t2679: f64, t9796: f64, t33308: f64, t9805: f64) -> (f64, f64, f64, f64) {
    let t43389 = t9800 * t33289 * t9806;
    let t43398 = 0.15337170381568299871e1_f64 * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    let t43403 = t9805 * t33308 * t9806;
    (t43389, t43398, t43400, t43403)
}
