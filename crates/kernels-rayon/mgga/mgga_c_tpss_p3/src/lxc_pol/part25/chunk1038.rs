//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1038/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1038(t4708: f64, t8167: f64, t10654: f64, t14252: f64, t14254: f64, t14258: f64, t14300: f64, t14304: f64, t14308: f64, t14311: f64, t14314: f64, t14316: f64, t761: f64, t771: f64, t797: f64, t8177: f64, t8188: f64) -> f64 {
    let t14318 = t8167 * t4708;
    let t14320 = -7.0_f64 / 2304.0_f64 * t14252 + 7.0_f64 / 4608.0_f64 * t14254 + 5.0_f64 / 768.0_f64 * t797 * t14258 - t771 * t14300 / 3072.0_f64 - t10654 - t761 * t14304 / 48.0_f64 - 35.0_f64 / 216.0_f64 * t8177 - t8188 - 35.0_f64 / 1152.0_f64 * t14308 - t797 * t14311 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t14314 + 7.0_f64 / 144.0_f64 * t14316 - 7.0_f64 / 48.0_f64 * t14318;
    t14320
}
