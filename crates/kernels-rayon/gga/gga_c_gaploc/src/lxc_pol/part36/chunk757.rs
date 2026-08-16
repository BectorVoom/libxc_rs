//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 757/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk757(t3338: f64, t447: f64, t2366: f64, t2754: f64, t874: f64, t6508: f64, t2293: f64, t986: f64, t197: f64, t161: f64, t10215: f64, t158: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31557 = t3338 * t447;
    let t31558 = t2366 * t31557;
    let t31585 = t2754 * t874;
    let t31586 = t6508 * t31585;
    let t31590 = t986 * t2293;
    let t31591 = t6508 * t31590;
    let t31730 = t197 * t3338;
    let t31731 = t31730 * t161;
    let t31740 = t158 * t10215;
    (t31558, t31586, t31590, t31591, t31730, t31731, t31740)
}
