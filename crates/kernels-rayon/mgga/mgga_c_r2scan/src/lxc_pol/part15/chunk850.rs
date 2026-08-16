//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 850/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk850(t216: f64, t5248: f64, t5253: f64, t5256: f64, t5258: f64, t5263: f64, t5274: f64, t5278: f64, t5282: f64, t5283: f64, t5288: f64, t5295: f64, t7007: f64) -> f64 {
    let t7681 = -0.21973736767207854065e-2_f64 * t7007 * t216 + t5248 - 0.8103123984e0_f64 * t5253 + 0.1350520664e0_f64 * t5256 + 0.20508037716432813316e4_f64 * t5258 + t5263 + t5274 - t5278 + t5282 - 0.11696447245269292414e1_f64 * t5283 - t5288 - t5295;
    t7681
}
