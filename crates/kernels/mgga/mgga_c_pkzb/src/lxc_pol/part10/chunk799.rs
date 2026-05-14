//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 799/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk799<F: Float>(t3880: F, t405: F, t921: F, t758: F, t2371: F, t3875: F, t154: F, t2352: F, t3757: F, t1167: F, t394: F) -> (F, F, F, F, F, F) {
    let t3881 = t405 * t3880;
    let t3882 = t3881 * t921;
    let t3883 = t758 * t3882;
    let t3886 = t3875 * t2371;
    let t3887 = t758 * t3886;
    let t3892 = t154 * t2352 * t3757;
    let t3898 = t394 * t1167;
    (t3882, t3883, t3886, t3887, t3892, t3898)
}
