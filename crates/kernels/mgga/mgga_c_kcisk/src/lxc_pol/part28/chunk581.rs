//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 581/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk581<F: Float>(t1801: F, t6684: F, t5062: F, t1869: F, t1757: F, t2527: F) -> (F, F, F, F) {
    let t6685 = t1801 * t6684;
    let t6686 = t5062 * t6685;
    let t6687 = t1869 * t6686;
    let t6689 = t2527 * t1757;
    (t6685, t6686, t6687, t6689)
}
