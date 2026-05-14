//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1212/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1212<F: Float>(t1376: F, t1707: F, t584: F, t5880: F, t591: F, t4885: F, t661: F, t1416: F, t1789: F, t1726: F, t5364: F, t607: F, t1819: F, t1821: F, t21430: F, t234: F) -> (F, F, F, F, F, F) {
    let t22355 = t1376 * t1707;
    let t22358 = t584 * t5880 * t591;
    let t22360 = t4885 * t661;
    let t22362 = t1416 * t1789;
    let t22365 = t1726 * t607 * t5364;
    let t22375 = 0.30762056574649219974e4 * t234 * t1819 * t1821 * t21430;
    (t22355, t22358, t22360, t22362, t22365, t22375)
}
