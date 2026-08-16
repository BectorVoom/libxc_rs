//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1217/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1217<F: Float>(t5977: F, t7997: F, t103220: F, t103234: F, t10871: F, t110453: F, t110459: F, t113163: F, t113269: F, t115499: F, t2061: F, t231: F, t23244: F, t23383: F, t23413: F, t25391: F, t25416: F, t26550: F, t27199: F, t2723: F, t30342: F, t30357: F, t7070: F, t7071: F, t7076: F, t93118: F, t93355: F, t95807: F) -> (F, F) {
    let t115573 = t7997 * t5977;
    let t115592 = F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t2061 * t23244 * t231 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t115499 * t2723 + F::cast_from(0.26020884564615598386e1_f64) * t7070 * t93355 * t115499 * t10871 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t26550 * t113163 - F::cast_from(0.77108554593144223218e-1_f64) * t110453 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t2061 * t23383 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t115573 * t2723 + t95807 + F::cast_from(0.39029762157531132076e-1_f64) * t103220 + F::cast_from(0.29272321618148349057e-1_f64) * t110459 + F::cast_from(0.52041769129231196772e1_f64) * t27199 * t30342 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t2061 * t23413 + F::cast_from(0.26020884564615598386e1_f64) * t27199 * t30357 - F::cast_from(0.72280234901709995519e-3_f64) * t103234 - F::cast_from(0.26020884564615598386e1_f64) * t25391 * t26550 * t113269;
    (t115573, t115592)
}
