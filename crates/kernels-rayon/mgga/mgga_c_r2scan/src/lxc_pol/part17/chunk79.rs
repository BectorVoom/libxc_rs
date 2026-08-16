//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 79/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk79(t159: f64, t171: f64, t180: f64, t192: f64, t216: f64, t230: f64, t234: f64, t236: f64, t41: f64, t61: f64, t15: f64, t151: f64) -> (f64, f64) {
    let t239 = 0.285764e-1_f64 * t159 * t171 + t180 - t192 - t41 * t230 - 0.21973736767207854065e-2_f64 * t61 * t216 + 0.5848223622634646207e0_f64 * t234 * t236;
    let t244 = 1.0_f64 + 0.4445e-1_f64 * t15 + t151;
    (t239, t244)
}
