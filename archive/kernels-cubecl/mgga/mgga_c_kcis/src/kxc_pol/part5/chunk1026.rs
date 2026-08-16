//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1026/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1026<F: Float>(t10513: F, t284: F, t41: F, t9545: F, t3436: F, t9588: F, t1094: F, t5163: F, t1780: F, t245: F, t3393: F, t5155: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t14832 = t10513 * t284;
    let t14838 = t41 * t9545;
    let t14849 = t9588 * t3436;
    let t14874 = t5163 * t1094;
    let t14875 = t14874 * sigma0;
    let t14907 = t1780 * t245;
    let t14913 = t3393 * t5155;
    (t14832, t14838, t14849, t14875, t14907, t14913)
}
