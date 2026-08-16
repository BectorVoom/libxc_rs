//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 75/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk75(t180: f64, t192: f64, t198: f64, t202: f64, t208: f64, t216: f64, t220: f64, t226: f64) -> f64 {
    let t229 = -0.6388517036e-2_f64 * t198 + 1.0_f64 * t202 * t208 + t180 - t192 - 0.21973736767207854065e-2_f64 * t216 + 0.5848223622634646207e0_f64 * t220 * t226;
    t229
}
