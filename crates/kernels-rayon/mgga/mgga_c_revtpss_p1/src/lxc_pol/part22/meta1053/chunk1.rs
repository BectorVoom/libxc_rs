//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3720/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720(t12916: f64, t21299: f64, t3718: f64, t17484: f64, t17748: f64, t17754: f64, t17756: f64, t3671: f64, t371: f64, t372: f64, t3720: f64, t482: f64, t57241: f64, t57250: f64, t57252: f64, t57256: f64, t57258: f64, t57382: f64, t59011: f64, t59196: f64, t70235: f64, t70511: f64, t70513: f64, t70521: f64, t70523: f64, t70530: f64) -> f64 {
    let t70542 = t3718 * t12916 * t21299;
    let t70546 = -0.95275595817932748827e-4_f64 * t70511 + 0.85748036236139473944e-3_f64 * t3671 * t371 * t372 * t482 * t70513 - 0.28582678745379824648e-3_f64 * t70521 + 0.57165357490759649296e-3_f64 * t70523 - 0.3811023832717309953e-3_f64 * t57241 + 0.19055119163586549765e-3_f64 * t57250 - 0.30488190661738479624e-2_f64 * t57252 + 0.19055119163586549765e-3_f64 * t57256 + 0.31758531939310916275e-3_f64 * t57258 + 0.42874018118069736972e-3_f64 * t70530 * t17756 + 0.30011812682648815881e-2_f64 * t59011 * t3720 * t70235 * t17748 - 0.21437009059034868486e-3_f64 * t59196 * t3720 * t70235 * t17754 - 0.28582678745379824648e-3_f64 * t70542 + 0.42874018118069736972e-3_f64 * t57382 * t17484;
    t70546
}
