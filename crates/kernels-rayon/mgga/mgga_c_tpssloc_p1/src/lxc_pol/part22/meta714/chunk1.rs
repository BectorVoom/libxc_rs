//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2318/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2318(t1519: f64, t5611: f64, t21013: f64, t814: f64, t20937: f64, t68: f64, t39249: f64, t39256: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t40673: f64, t40679: f64, t46138: f64, t67044: f64, t67086: f64, t67087: f64, t67088: f64, t67089: f64, t67090: f64, t67095: f64, t67096: f64) -> (f64, f64, f64, f64) {
    let t67405 = t1519 * t5611;
    let t67429 = t814 * t21013;
    let t67441 = t20937 * t68;
    let t67448 = -t39249 - t67044 + t67086 - t39256 - t67087 + t67088 - t67089 + t67090 + t46138 + t67095 - t39309 + t39312 + t39316 + t39320 - t67096 + t40673 - t40679;
    (t67405, t67429, t67441, t67448)
}
