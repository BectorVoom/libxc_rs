//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 612/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk612<F: Float>(t1895: F, t6974: F, t1869: F, t1900: F, t6719: F, t1636: F, t2571: F) -> (F, F, F, F, F) {
    let t6975 = t6974 * t1895;
    let t6976 = t1869 * t6975;
    let t6978 = t6719 * t1900;
    let t6979 = t1869 * t6978;
    let t6981 = t2571 * t1636;
    (t6975, t6976, t6978, t6979, t6981)
}
