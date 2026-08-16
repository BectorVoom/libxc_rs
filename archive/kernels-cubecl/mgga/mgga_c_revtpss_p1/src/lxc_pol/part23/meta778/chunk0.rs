//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2582/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582<F: Float>(t12553: F, t300: F, t3521: F, t1261: F, t1715: F, t247: F, t44701: F, t1247: F, t1796: F, t42994: F, t3718: F, t44546: F, t5347: F) -> (F, F, F, F, F) {
    let t58672 = t300 * t12553;
    let t58708 = t300 * t3521;
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58824 = t1247 * t42994 * t1796;
    let t58850 = t3718 * t44546 * t5347;
    (t58672, t58708, t58777, t58824, t58850)
}
