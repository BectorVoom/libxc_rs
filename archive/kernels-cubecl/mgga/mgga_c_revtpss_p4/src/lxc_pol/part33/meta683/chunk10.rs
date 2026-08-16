//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2250/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2250<F: Float>(t21169: F, t7607: F, t816: F, t8171: F, t104894: F, t104905: F, t20266: F, t20293: F, t20306: F, t20310: F, t20868: F, t21004: F, t21184: F, t26852: F, t26880: F, t29047: F, t29048: F, t29054: F, t29055: F, t57549: F, t6679: F, t7624: F, t97179: F) -> (F, F) {
    let t112397 = t7607 * t21169;
    let t112404 = t8171 * t816;
    let t112424 = t112397 / F::cast_from(648.0_f64) - F::cast_from(0.28582678745379824648e-3_f64) * t26852 * t6679 - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t29047 * t57549 * t20293 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t112404 * t29055 + t29047 * t29054 * t20266 / F::cast_from(216.0_f64) - t29047 * t29048 * t20310 / F::cast_from(72.0_f64) - t29047 * t29048 * t20306 / F::cast_from(48.0_f64) + F::cast_from(0.17149607247227894789e-2_f64) * t97179 * t21004 - F::cast_from(0.7622047665434619906e-3_f64) * t104894 + F::cast_from(0.28582678745379824648e-2_f64) * t7624 * t20868 - F::cast_from(0.19055119163586549765e-3_f64) * t104905 + F::cast_from(0.28582678745379824648e-3_f64) * t26880 * t21184;
    (t112404, t112424)
}
