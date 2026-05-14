//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1200/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1200<F: Float>(t18000: F, t21454: F, t18009: F, t5984: F, t7637: F, t7716: F, t7725: F, t7713: F, t2064: F, t2899: F, t2902: F, t154: F, t2048: F, t276: F, t7350: F, t1066: F, t18060: F) -> (F, F, F, F, F, F, F, F) {
    let t21462 = t18000 * t21454;
    let t21468 = t18009 * t21454;
    let t21485 = t5984 * t7637;
    let t21494 = t7725 * t7716;
    let t21496 = t5984 * t7713;
    let t21499 = t2899 * t2064 * t2902;
    let t21527 = t276 * t154 * t2048 * t7350;
    let t21538 = t276 * t154 * t18060 * t1066;
    (t21462, t21468, t21485, t21494, t21496, t21499, t21527, t21538)
}
