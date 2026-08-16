//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 817/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk817(t2617: f64, t3451: f64, t7803: f64, t13069: f64, t7416: f64, t10040: f64, t25198: f64, t11112: f64, t2679: f64, t9800: f64, t13055: f64, t5640: f64) -> (f64, f64, f64, f64, f64) {
    let t43609 = t7803 * t3451 * t2617;
    let t43611 = t7416 * t13069;
    let t43646 = t25198 * t10040;
    let t43650 = t9800 * t11112 * t2679;
    let t43652 = t5640 * t13055;
    (t43609, t43611, t43646, t43650, t43652)
}
