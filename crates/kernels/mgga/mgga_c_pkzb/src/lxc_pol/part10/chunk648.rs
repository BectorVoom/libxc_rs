//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 648/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk648<F: Float>(t2746: F, t685: F, t1084: F, t1850: F, t683: F, t1855: F, t1073: F, t1861: F, t667: F, t1833: F, t1865: F, t2730: F, t2741: F) -> (F, F, F, F, F, F, F) {
    let t2748 = 1.0 * t2746 * t685;
    let t2750 = 1.0 * t1850 * t1084;
    let t2751 = t1084 * t683;
    let t2753 = 2.0 * t1855 * t2751;
    let t2754 = t1861 * t1073;
    let t2755 = t2754 * t667;
    let t2759 = t1865 - t1833 / 3.0 - t2730 / 3.0 + t2741;
    (t2748, t2750, t2751, t2753, t2754, t2755, t2759)
}
