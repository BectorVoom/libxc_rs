//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1184/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1184<F: Float>(t12322: F, t21910: F, t1395: F, t1464: F, t15808: F, t1947: F, t1394: F, t17261: F, t5644: F, t4160: F, t17292: F, t5649: F, t5655: F, t20974: F, t5662: F, t4162: F) -> (F, F, F, F, F, F) {
    let t21911 = t12322 * t21910;
    let t21912 = t1395 * t21911;
    let t21913 = t1464 * t21912;
    let t21918 = t15808 * t1947;
    let t21919 = t1394 * t21918;
    let t21922 = t17261 * t5644;
    let t21923 = t4160 * t21922;
    let t21925 = t17292 * t5649;
    let t21926 = t4160 * t21925;
    let t21928 = t17292 * t5655;
    let t21929 = t4160 * t21928;
    let t21931 = t5662 * t20974;
    let t21932 = t4162 * t21931;
    (t21913, t21919, t21923, t21926, t21929, t21932)
}
