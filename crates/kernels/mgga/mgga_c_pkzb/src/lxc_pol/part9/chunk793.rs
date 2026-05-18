//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 793/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk793<F: Float>(t179: F, t5634: F, t5635: F, t5537: F, t780: F, t154: F, t1843: F, t2048: F, t276: F, t742: F, t2003: F, t52: F) -> (F, F, F, F, F, F) {
    let t5637 = t179 * t5634 * t5635;
    let t5641 = t179 * t780 * t5537;
    let t5645 = t154 * t2048 * t1843;
    let t5646 = t276 * t5645;
    let t5649 = t154 * t742 * t5537;
    let t5656 = t52 * t2003;
    (t5637, t5641, t5645, t5646, t5649, t5656)
}
