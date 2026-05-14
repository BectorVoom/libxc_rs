//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 885/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk885<F: Float>(t140: F, t5598: F, t6672: F, t15909: F, t6675: F, t5192: F, t2063: F, t3293: F) -> (F, F, F) {
    let t15916 = t140 * t5598 * t6672;
    let t15917 = t6675 * t15909;
    let t15918 = t5192 * t15917;
    let t15919 = t15916 * t15918;
    let t15921 = t2063 * t3293;
    (t15917, t15919, t15921)
}
