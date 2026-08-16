//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1195/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1195(t37920: f64, t39831: f64, t39832: f64, t39836: f64, t39855: f64, t41582: f64, t43266: f64, t43269: f64, t43271: f64, t43273: f64, t43275: f64, t43277: f64) -> f64 {
    let t43279 = t39831 - 0.14282990759302185292e-1_f64 * t39832 + 0.27439371595564631661e-2_f64 * t43266 - t37920 - 0.65495539973149862688e-2_f64 * t43269 - 0.13099107994629972538e-1_f64 * t43271 - 0.52396431978519890152e-1_f64 * t43273 + 0.21831846657716620896e-2_f64 * t43275 + t39836 + 0.54878743191129263322e-2_f64 * t43277 - t41582 - t39855;
    t43279
}
