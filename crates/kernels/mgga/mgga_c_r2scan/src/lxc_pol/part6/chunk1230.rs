//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1230/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1230<F: Float>(t2147: F, t2148: F, t22786: F, t2182: F, t3303: F, t481: F, t6584: F, t146: F, t6533: F, t774: F, t6537: F, t2150: F, t6856: F, t1568: F, t20306: F, t6155: F) -> (F, F, F, F, F, F, F) {
    let t22788 = t2147 * t2148 * t22786;
    let t22790 = t2182 * t3303;
    let t22791 = t6584 * t481;
    let t22793 = t22790 * t2148 * t22791;
    let t22796 = t146 * t6533 * t774;
    let t22797 = t22796 * t6537;
    let t22800 = t2147 * t6856 * t2150;
    let t22803 = t6155 * t1568 * t20306;
    (t22788, t22790, t22793, t22796, t22797, t22800, t22803)
}
