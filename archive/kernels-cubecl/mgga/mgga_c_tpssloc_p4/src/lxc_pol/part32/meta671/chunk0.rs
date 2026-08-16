//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2104/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2104<F: Float>(t24649: F, t27710: F, t23508: F, t8026: F, t27628: F, t7324: F, t7331: F, t15730: F, t7339: F, t24661: F, t27491: F, t24668: F, t27497: F) -> (F, F, F, F, F, F, F) {
    let t95323 = t27710 * t24649;
    let t95326 = t8026 * t23508;
    let t95332 = t7324 * t27628;
    let t95334 = F::cast_from(0.20186378047070195428e-3_f64) * t95332 * t7331;
    let t95335 = t7339 * t15730;
    let t95340 = t24661 * t27491;
    let t95346 = t24668 * t27497;
    (t95323, t95326, t95332, t95334, t95335, t95340, t95346)
}
