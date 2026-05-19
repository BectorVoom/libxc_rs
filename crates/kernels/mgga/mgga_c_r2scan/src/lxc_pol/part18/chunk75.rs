//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 75/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk75<F: Float>(t180: F, t192: F, t198: F, t202: F, t208: F, t216: F, t220: F, t226: F) -> F {
    let t229 = -F::cast_from(0.6388517036e-2_f64) * t198 + F::new(1.0) * t202 * t208 + t180 - t192 - F::cast_from(0.21973736767207854065e-2_f64) * t216 + F::cast_from(0.5848223622634646207e0_f64) * t220 * t226;
    t229
}
