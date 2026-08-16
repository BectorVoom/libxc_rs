//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1332/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1332(t1011: f64, t6218: f64, t225: f64, t29624: f64, t29614: f64, t24826: f64, t29782: f64, t29736: f64, t86094: f64, t131: f64, t467: f64, t5415: f64, t6794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103515 = t6218 * t1011;
    let t103520 = t29624 * t225;
    let t103528 = t29614 * t225;
    let t103546 = t24826 * t29782;
    let t103573 = t86094 * t29736;
    let t103581 = t5415 * t6794 * t131 * t467;
    (t103515, t103520, t103528, t103546, t103573, t103581)
}
