//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 843/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk843<F: Float>(t216: F, t5360: F, t5366: F, t5373: F, t5378: F, t7720: F, t7721: F, t7724: F, t7725: F, t7727: F, t7730: F, t7737: F, t8590: F) -> F {
    let t8934 = -t5360 + t7720 - F::cast_from(0.21973736767207854065e-2_f64) * t8590 * t216 + F::cast_from(0.20508037716432813315e4_f64) * t7721 - t7724 - F::cast_from(0.46785788981077169656e1_f64) * t7725 - F::cast_from(0.2602459512072417562e0_f64) * t7727 + t7730 + t5366 + F::new(0.1714584e0) * t5373 + F::cast_from(0.80040858019733333332e-2_f64) * t5378 + F::cast_from(0.1301229756036208781e0_f64) * t7737;
    t8934
}
