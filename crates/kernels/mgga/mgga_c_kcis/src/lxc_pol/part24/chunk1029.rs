//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1029/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1029<F: Float>(t1141: F, t7738: F, t2183: F, t3329: F, t1169: F, t982: F, t283: F, t3463: F) -> (F, F, F, F) {
    let t26868 = t7738 * t1141;
    let t26871 = t2183 * t3329;
    let t26891 = t1169 * t982;
    let t26896 = t3463 * t283;
    (t26868, t26871, t26891, t26896)
}
