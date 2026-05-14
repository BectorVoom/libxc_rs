//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 616/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk616<F: Float>(t204: F, t205: F, t2739: F, t1831: F, t1833: F, t2730: F, t228: F, t1070: F, t663: F, t685: F, t1084: F, t1850: F, t683: F, t1855: F, t1073: F, t1861: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2741 = t204 * t205 * t2739;
    let t2743 = t1831 - 0.17808333333333333333e-1 * t1833 - 0.17808333333333333333e-1 * t2730 + 0.53425e-1 * t2741;
    let t2745 = 0.621814e-1 * t2743 * t228;
    let t2746 = t1070 * t663;
    let t2748 = 1.0 * t2746 * t685;
    let t2750 = 1.0 * t1850 * t1084;
    let t2751 = t1084 * t683;
    let t2753 = 2.0 * t1855 * t2751;
    let t2754 = t1861 * t1073;
    (t2741, t2743, t2745, t2746, t2748, t2750, t2751, t2753, t2754)
}
