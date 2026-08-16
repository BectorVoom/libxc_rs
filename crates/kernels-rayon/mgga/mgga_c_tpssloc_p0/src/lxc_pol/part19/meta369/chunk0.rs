//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1360/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1360(t363: f64, t42342: f64, t42345: f64, t43288: f64, t3131: f64, t3047: f64, t3077: f64, t10908: f64, t3114: f64, t1036: f64, t10438: f64, t221: f64, t339: f64, t42813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43298 = t3077 * t3047;
    let t43301 = t3114 * t10908;
    let t43303 = t10438 * t1036;
    let t43307 = 5.0_f64 / 486.0_f64 * t339 * t221 * t42813;
    (t43291, t43292, t43298, t43301, t43303, t43307)
}
