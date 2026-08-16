//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1325/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1325<F: Float>(t21905: F, t4135: F, t1468: F, t1464: F, t1497: F, t6922: F, t12322: F, t1395: F, t15808: F, t1947: F, t1394: F, t17261: F, t5644: F) -> (F, F, F, F, F) {
    let t21906 = t4135 * t21905;
    let t21907 = t1468 * t21906;
    let t21908 = t1464 * t21907;
    let t21910 = t6922 * t1497;
    let t21911 = t12322 * t21910;
    let t21912 = t1395 * t21911;
    let t21913 = t1464 * t21912;
    let t21918 = t15808 * t1947;
    let t21919 = t1394 * t21918;
    let t21922 = t17261 * t5644;
    (t21908, t21910, t21913, t21919, t21922)
}
