//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 826/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk826<F: Float>(t13614: F, t397: F, t403: F, t396: F, t1390: F, t301: F, t1310: F, t1311: F, t164: F) -> (F, F, F, F) {
    let t13871 = t397 * t13614 * t403;
    let t13873 = 0.19989765240197019125e-1 * t396 * t13871;
    let t13893 = 1.0 / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13900 = t164 * t1311;
    (t13873, t13893, t13894, t13900)
}
