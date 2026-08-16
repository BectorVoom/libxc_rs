//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1328/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1328(t16804: f64, t252: f64, t1492: f64, t4265: f64, t225: f64, t5632: f64, t5561: f64, t1519: f64, t4142: f64, t5631: f64, t798: f64, t5558: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17083 = t16804 * t252;
    let t17087 = t1492 * t4265;
    let t17090 = t5632 * t225;
    let t17092 = t5561 * t225;
    let t17095 = t4142 * t1519;
    let t17098 = t798 * t5631;
    let t17100 = t5558 * t852;
    (t17083, t17087, t17090, t17092, t17095, t17098, t17100)
}
