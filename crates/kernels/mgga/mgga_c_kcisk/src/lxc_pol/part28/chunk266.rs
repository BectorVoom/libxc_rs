//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 266/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk266<F: Float>(t1224: F, t1636: F, t1697: F, t1696: F, t617: F) -> (F, F, F, F) {
    let t1699 = t1224 * t1697 * t1636;
    let t1701 = -t1696 - 0.17808333333333333333e-1 * t1699;
    let t1704 = t617 * t617;
    let t1705 = 1.0 / t1704;
    (t1699, t1701, t1704, t1705)
}
