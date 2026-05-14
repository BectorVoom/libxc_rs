//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 255/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk255<F: Float>(t1663: F, t1664: F, t1645: F, t1634: F, t1638: F) -> (F, F, F, F) {
    let t1665 = t1663 * t1664;
    let t1667 = 1.0 * t1645 * t1665;
    let t1668 = 0.92708333333333333333e-2 * t1634;
    let t1670 = -t1668 - 0.92708333333333333333e-2 * t1638;
    (t1665, t1667, t1668, t1670)
}
