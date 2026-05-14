//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1097/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1097<F: Float>(t4804: F, t717: F, t415: F, t20: F, t4982: F, t654: F, t1693: F, t1782: F, t4826: F) -> (F, F, F, F, F) {
    let t32927 = t717 * t4804;
    let t32928 = t415 * t32927;
    let t32931 = t4982 * t654 * t20;
    let t32932 = t1693 * t32931;
    let t32935 = t1782 * t4826;
    (t32927, t32928, t32931, t32932, t32935)
}
