//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1217/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1217(t5977: f64, t7997: f64, t103220: f64, t103234: f64, t10871: f64, t110453: f64, t110459: f64, t113163: f64, t113269: f64, t115499: f64, t2061: f64, t231: f64, t23244: f64, t23383: f64, t23413: f64, t25391: f64, t25416: f64, t26550: f64, t27199: f64, t2723: f64, t30342: f64, t30357: f64, t7070: f64, t7071: f64, t7076: f64, t93118: f64, t93355: f64, t95807: f64) -> (f64, f64) {
    let t115573 = t7997 * t5977;
    let t115592 = 0.4336814094102599731e0_f64 * t7070 * t7076 * t2061 * t23244 * t231 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t115499 * t2723 + 0.26020884564615598386e1_f64 * t7070 * t93355 * t115499 * t10871 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t113163 - 0.77108554593144223218e-1_f64 * t110453 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t2061 * t23383 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t115573 * t2723 + t95807 + 0.39029762157531132076e-1_f64 * t103220 + 0.29272321618148349057e-1_f64 * t110459 + 0.52041769129231196772e1_f64 * t27199 * t30342 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t2061 * t23413 + 0.26020884564615598386e1_f64 * t27199 * t30357 - 0.72280234901709995519e-3_f64 * t103234 - 0.26020884564615598386e1_f64 * t25391 * t26550 * t113269;
    (t115573, t115592)
}
