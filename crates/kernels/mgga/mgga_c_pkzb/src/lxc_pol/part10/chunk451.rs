//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 451/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk451<F: Float>(t1634: F, t50: F, t581: F, t1692: F, t165: F, t586: F, t158: F) -> (F, F, F) {
    let t1707 = t50 * t1634;
    let t1708 = t581 * t1707;
    let t1712 = t581 * t50 * t1692;
    let t1716 = 1.0 / t586 / t165;
    let t1717 = t158 * t1716;
    (t1708, t1712, t1717)
}
