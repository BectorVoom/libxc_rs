//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 849/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk849<F: Float>(t216: F, t5248: F, t5253: F, t5256: F, t5258: F, t5263: F, t5274: F, t5278: F, t5282: F, t5283: F, t5288: F, t5295: F, t7007: F) -> F {
    let t7681 = -F::cast_from(0.21973736767207854065e-2_f64) * t7007 * t216 + t5248 - F::cast_from(0.8103123984e0_f64) * t5253 + F::cast_from(0.1350520664e0_f64) * t5256 + F::cast_from(0.20508037716432813316e4_f64) * t5258 + t5263 + t5274 - t5278 + t5282 - F::cast_from(0.11696447245269292414e1_f64) * t5283 - t5288 - t5295;
    t7681
}
