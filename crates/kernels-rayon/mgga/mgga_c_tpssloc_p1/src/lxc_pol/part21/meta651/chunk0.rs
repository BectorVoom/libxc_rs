//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2447/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2447(t1015: f64, t10472: f64, t42559: f64, t10870: f64, t3048: f64, t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64, t10510: f64, t3109: f64) -> (f64, f64, f64, f64, f64) {
    let t43211 = t10472 * t1015 * t42559;
    let t43214 = t3048 * t10870;
    let t43216 = t204 * t376;
    let t43219 = t1020 * t248 * t43216 * t1023;
    let t43221 = t3109 * t10510;
    (t43211, t43214, t43216, t43219, t43221)
}
