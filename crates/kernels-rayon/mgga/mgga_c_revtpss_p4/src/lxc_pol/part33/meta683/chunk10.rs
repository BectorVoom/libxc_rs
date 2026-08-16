//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2250/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2250(t21169: f64, t7607: f64, t816: f64, t8171: f64, t104894: f64, t104905: f64, t20266: f64, t20293: f64, t20306: f64, t20310: f64, t20868: f64, t21004: f64, t21184: f64, t26852: f64, t26880: f64, t29047: f64, t29048: f64, t29054: f64, t29055: f64, t57549: f64, t6679: f64, t7624: f64, t97179: f64) -> (f64, f64) {
    let t112397 = t7607 * t21169;
    let t112404 = t8171 * t816;
    let t112424 = t112397 / 648.0_f64 - 0.28582678745379824648e-3_f64 * t26852 * t6679 - 7.0_f64 / 648.0_f64 * t29047 * t57549 * t20293 - 2.0_f64 / 81.0_f64 * t112404 * t29055 + t29047 * t29054 * t20266 / 216.0_f64 - t29047 * t29048 * t20310 / 72.0_f64 - t29047 * t29048 * t20306 / 48.0_f64 + 0.17149607247227894789e-2_f64 * t97179 * t21004 - 0.7622047665434619906e-3_f64 * t104894 + 0.28582678745379824648e-2_f64 * t7624 * t20868 - 0.19055119163586549765e-3_f64 * t104905 + 0.28582678745379824648e-3_f64 * t26880 * t21184;
    (t112404, t112424)
}
