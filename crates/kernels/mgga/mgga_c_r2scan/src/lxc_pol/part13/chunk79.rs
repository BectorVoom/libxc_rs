//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 79/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk79<F: Float>(t159: F, t171: F, t180: F, t192: F, t216: F, t230: F, t234: F, t236: F, t41: F, t61: F, t15: F, t151: F) -> (F, F) {
    let t239 = F::new(0.285764e-1) * t159 * t171 + t180 - t192 - t41 * t230 - F::cast_from(0.21973736767207854065e-2_f64) * t61 * t216 + F::cast_from(0.5848223622634646207e0_f64) * t234 * t236;
    let t244 = F::new(1.0) + F::new(0.4445e-1) * t15 + t151;
    (t239, t244)
}
