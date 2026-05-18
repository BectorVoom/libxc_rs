//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 817/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk817<F: Float>(t8590: F, t86: F, t2625: F, t2854: F, t2858: F, t3232: F, t797: F, t2266: F, t481: F, t2333: F) -> (F, F, F, F) {
    let t8591 = t8590 * t86;
    let t8592 = F::new(0.19751673498613801407e-1) * t8591;
    let t8595 = t2858 * t2854 * t2625;
    let t8596 = F::new(12.0) * t8595;
    let t8597 = t3232 * t797;
    let t8599 = t2266 * t8597 * t481;
    let t8600 = F::new(3.0) * t8599;
    let t8601 = t3232 * t2333;
    (t8592, t8596, t8600, t8601)
}
