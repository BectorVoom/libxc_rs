//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2284/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2284(t17673: f64, t17984: f64, t25589: f64, t4596: f64, t4600: f64, t7578: f64, t83054: f64, t83058: f64, t88320: f64, t88321: f64, t88324: f64, t88335: f64, t88336: f64, t88339: f64, t88594: f64, t88600: f64) -> f64 {
    let t99571 = t83054 * t17673 / 256.0_f64 - t83058 * t17984 / 256.0_f64 + t88594 * t4596 / 384.0_f64 - t88600 * t4600 / 768.0_f64 - 0.20186378047070195428e-3_f64 * t25589 * t7578 + t88320 - t88321 / 5184.0_f64 + t88324 - t88335 - t88336 / 648.0_f64 + t88339;
    t99571
}
