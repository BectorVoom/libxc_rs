//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3720/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3720<F: Float>(t12916: F, t21299: F, t3718: F, t17484: F, t17748: F, t17754: F, t17756: F, t3671: F, t371: F, t372: F, t3720: F, t482: F, t57241: F, t57250: F, t57252: F, t57256: F, t57258: F, t57382: F, t59011: F, t59196: F, t70235: F, t70511: F, t70513: F, t70521: F, t70523: F, t70530: F) -> F {
    let t70542 = t3718 * t12916 * t21299;
    let t70546 = -F::cast_from(0.95275595817932748827e-4_f64) * t70511 + F::cast_from(0.85748036236139473944e-3_f64) * t3671 * t371 * t372 * t482 * t70513 - F::cast_from(0.28582678745379824648e-3_f64) * t70521 + F::cast_from(0.57165357490759649296e-3_f64) * t70523 - F::cast_from(0.3811023832717309953e-3_f64) * t57241 + F::cast_from(0.19055119163586549765e-3_f64) * t57250 - F::cast_from(0.30488190661738479624e-2_f64) * t57252 + F::cast_from(0.19055119163586549765e-3_f64) * t57256 + F::cast_from(0.31758531939310916275e-3_f64) * t57258 + F::cast_from(0.42874018118069736972e-3_f64) * t70530 * t17756 + F::cast_from(0.30011812682648815881e-2_f64) * t59011 * t3720 * t70235 * t17748 - F::cast_from(0.21437009059034868486e-3_f64) * t59196 * t3720 * t70235 * t17754 - F::cast_from(0.28582678745379824648e-3_f64) * t70542 + F::cast_from(0.42874018118069736972e-3_f64) * t57382 * t17484;
    t70546
}
