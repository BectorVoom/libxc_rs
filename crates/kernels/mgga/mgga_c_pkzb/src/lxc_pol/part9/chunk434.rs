//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 434/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk434<F: Float>(t158: F, t1739: F, t1740: F, t133: F, t614: F, t1634: F, t1692: F, t596: F, t160: F, t162: F, t594: F, t597: F) -> (F, F, F, F) {
    let t1742 = (t1739 + t1740) * t158;
    let t1746 = t133 * t614;
    let t1747 = t1746 * t1634;
    let t1750 = t596 * t1692;
    let t1753 = -12.0 * t160 * t1747 + 3.0 * t160 * t1750 - t162 * t1742 + 6.0 * t594 * t597;
    (t1742, t1747, t1750, t1753)
}
