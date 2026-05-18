//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 931/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk931<F: Float>(t19763: F, t3338: F, t3337: F, t10707: F, t6693: F, t5053: F, t5083: F, t10745: F, t6720: F, t19756: F, t5077: F, t5076: F) -> (F, F, F, F, F, F, F) {
    let t19870 = t3338 * t19763;
    let t19871 = t3337 * t19870;
    let t19873 = t10707 * t6693;
    let t19875 = t5083 * t5053;
    let t19877 = t10745 * t6720;
    let t19879 = t5077 * t19756;
    let t19880 = t5076 * t19879;
    (t19870, t19871, t19873, t19875, t19877, t19879, t19880)
}
