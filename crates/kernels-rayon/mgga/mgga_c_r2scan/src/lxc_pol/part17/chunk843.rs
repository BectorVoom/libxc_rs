//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 843/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk843(t216: f64, t5360: f64, t5366: f64, t5373: f64, t5378: f64, t7720: f64, t7721: f64, t7724: f64, t7725: f64, t7727: f64, t7730: f64, t7737: f64, t8590: f64) -> f64 {
    let t8934 = -t5360 + t7720 - 0.21973736767207854065e-2_f64 * t8590 * t216 + 0.20508037716432813315e4_f64 * t7721 - t7724 - 0.46785788981077169656e1_f64 * t7725 - 0.2602459512072417562e0_f64 * t7727 + t7730 + t5366 + 0.1714584e0_f64 * t5373 + 0.80040858019733333332e-2_f64 * t5378 + 0.1301229756036208781e0_f64 * t7737;
    t8934
}
