//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1709/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1709(t23097: f64, t28396: f64, t1516: f64, t25068: f64, t5624: f64, t6621: f64, t5572: f64, t6581: f64, t16758: f64, t232: f64, t6646: f64, t1888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28397 = t23097 * t28396;
    let t28399 = t25068 * t1516;
    let t28401 = t6621 * t5624;
    let t28403 = t6581 * t5572;
    let t28418 = t16758 * t232;
    let t28419 = t6646 * t28418;
    let t28420 = t1888 * t28419;
    (t28397, t28399, t28401, t28403, t28418, t28419, t28420)
}
