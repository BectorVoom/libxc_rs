//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 675/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk675<F: Float>(t6759: F, t7234: F, t1785: F, t2364: F, t5015: F, t5014: F, t662: F) -> (F, F, F, F) {
    let t7235 = t7234 * t6759;
    let t7238 = t2364 * t1785;
    let t7239 = t5015 * t7238;
    let t7242 = t5014 * t662;
    (t7235, t7238, t7239, t7242)
}
