//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1380/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1380<F: Float>(t14680: F, t26871: F, t3331: F, t33862: F, t8064: F, t2189: F, t46015: F, t26868: F, t5189: F, t14668: F, t26886: F, t3330: F, t3481: F, t8081: F) -> (F, F, F, F, F, F) {
    let t97507 = F::new(4.0) * t26871 * t14680;
    let t97510 = F::new(24.0) * t33862 * t8064 * t3331;
    let t97511 = t46015 * t2189;
    let t97513 = F::new(2.0) * t26868 * t5189;
    let t97517 = F::new(2.0) * t14668 * t26886;
    let t97521 = F::new(2.0) * t3330 * t8081 * t3481;
    (t97507, t97510, t97511, t97513, t97517, t97521)
}
