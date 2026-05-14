//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 75/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk75<F: Float>(t180: F, t192: F, t198: F, t202: F, t208: F, t216: F, t220: F, t226: F) -> (F,) {
    let t229 = -0.6388517036e-2 * t198 + 1.0 * t202 * t208 + t180 - t192 - 0.21973736767207854065e-2 * t216 + 0.5848223622634646207e0 * t220 * t226;
    (t229,)
}
