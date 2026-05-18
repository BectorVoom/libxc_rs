//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 587/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk587<F: Float>(t481: F, t797: F, t3262: F, t3263: F, t106: F, t494: F, t97: F) -> (F, F, F, F) {
    let t3264 = t797 * t481;
    let t3266 = t3262 * t3263 * t3264;
    let t3267 = F::new(3.0) / F::new(4.0) * t3266;
    let t3268 = t106 * t494;
    let t3269 = t97 * t3268;
    (t3264, t3267, t3268, t3269)
}
