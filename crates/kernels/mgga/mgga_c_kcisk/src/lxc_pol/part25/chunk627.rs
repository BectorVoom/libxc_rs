//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 627/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk627<F: Float>(t6667: F, t6675: F, t5192: F, t6674: F, t1636: F, t2563: F, t5184: F, t5182: F, t1894: F, t2527: F, t1801: F, t5062: F, t1869: F, t1757: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6676 = t6675 * t6667;
    let t6677 = t5192 * t6676;
    let t6678 = t6674 * t6677;
    let t6680 = t2563 * t1636;
    let t6681 = t5184 * t6680;
    let t6682 = t5182 * t6681;
    let t6684 = t2527 * t1894;
    let t6685 = t1801 * t6684;
    let t6686 = t5062 * t6685;
    let t6687 = t1869 * t6686;
    let t6689 = t2527 * t1757;
    (t6676, t6677, t6678, t6680, t6681, t6682, t6684, t6685, t6686, t6687, t6689)
}
