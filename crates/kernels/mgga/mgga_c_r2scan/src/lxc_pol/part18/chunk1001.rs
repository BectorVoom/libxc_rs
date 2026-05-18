//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1001/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1001<F: Float>(t12412: F, t2892: F, t797: F, t3263: F, t10610: F, t11479: F, t3262: F, t3574: F, t106: F, t3055: F, t97: F) -> (F, F, F, F, F, F) {
    let t12413 = t12412 / F::new(2.0);
    let t12414 = t797 * t2892;
    let t12415 = t3263 * t12414;
    let t12416 = t10610 * t12415;
    let t12417 = F::new(3.0) / F::new(2.0) * t12416;
    let t12419 = t3262 * t11479 * t3574;
    let t12420 = F::new(3.0) / F::new(2.0) * t12419;
    let t12422 = t97 * t106 * t3055;
    (t12413, t12414, t12415, t12417, t12420, t12422)
}
