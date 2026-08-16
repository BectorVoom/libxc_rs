//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 758/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk758(t10425: f64, t10510: f64, t11235: f64, t11687: f64, t752: f64, t1907: f64, t5211: f64, t1957: f64, t1904: f64, t5217: f64, t5219: f64, t5213: f64, t5339: f64) -> (f64, f64, f64, f64) {
    let t11689 = t10425 + t10510 + t11235 + t11687;
    let t11690 = t11689 * t752;
    let t11691 = t5211 * t1907;
    let t11693 = 3.0_f64 * t11691 * t1957;
    let t11694 = t1904 * t5217;
    let t11696 = 6.0_f64 * t11694 * t5219;
    let t11698 = 3.0_f64 * t5213 * t5339;
    (t11690, t11693, t11696, t11698)
}
