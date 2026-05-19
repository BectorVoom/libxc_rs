//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1122/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1122<F: Float>(t1065: F, t2847: F, t11002: F, t3274: F, t5086: F, t97: F, t10935: F, t2813: F, t3446: F, t3261: F, t498: F, t10648: F, t10971: F, t11564: F) -> (F, F, F, F, F) {
    let t40589 = t1065 * t2847;
    let t40590 = t11002 * t40589;
    let t40594 = t97 * t3274 * t5086;
    let t40603 = t3446 * t10935 * t2813;
    let t40604 = F::cast_from(0.19211284388664477842e-2_f64) * t40603;
    let t40630 = t97 * t3261 * t498;
    let t40642 = t10648 * t10971 * t11564;
    (t40590, t40594, t40604, t40630, t40642)
}
