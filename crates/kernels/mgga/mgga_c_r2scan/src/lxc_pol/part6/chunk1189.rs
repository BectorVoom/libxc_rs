//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1189/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1189<F: Float>(t1907: F, t5382: F, t644: F, t21358: F, t5380: F, t621: F, t21384: F, t5832: F, t5583: F, t632: F, t21066: F, t5447: F, t653: F, t1893: F, t5388: F, t5686: F) -> (F, F, F, F, F, F) {
    let t21829 = 0.11579025239058625248e4 * t1907 * t644 * t5382;
    let t21832 = 0.3859675079686208416e3 * t5380 * t21358 * t621;
    let t21837 = 0.17096e1 * t5832 * t21384;
    let t21843 = 8.0 * t632 * t5583 * t621;
    let t21846 = 0.38596750796862084161e4 * t5447 * t653 * t21066;
    let t21858 = 0.2069040516770936012e4 * t5388 * t1893 * t5686 * t621;
    (t21829, t21832, t21837, t21843, t21846, t21858)
}
