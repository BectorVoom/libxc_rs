//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1981/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1981(t5611: f64, t852: f64, t17100: f64, t225: f64, t17087: f64, t17060: f64, t17095: f64, t17098: f64, t18287: f64, t1176: f64, t1714: f64, t19256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t59331 = t852 * t5611;
    let t59466 = t17100 * t225;
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t64595 = t18287 * t225;
    let t64825 = t1176 * t1714;
    let t65203 = t19256 * t225;
    (t59331, t59466, t59498, t59503, t59519, t59537, t64595, t64825, t65203)
}
