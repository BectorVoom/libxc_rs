//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2209/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2209(t17022: f64, t814: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64, t17095: f64, t17098: f64, t10143: f64, t5660: f64, t17109: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t59347 = t814 * t17022;
    let t59466 = t17100 * t225;
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t59564 = t5660 * t10143;
    let t59584 = t17109 * t2752;
    (t59347, t59466, t59498, t59503, t59519, t59537, t59564, t59584)
}
