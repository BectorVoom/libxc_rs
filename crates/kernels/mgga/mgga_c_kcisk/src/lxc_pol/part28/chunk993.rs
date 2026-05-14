//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 993/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk993<F: Float>(t23029: F, t5192: F, t15916: F, t719: F, t8831: F, t1894: F, t1873: F, t1869: F, t642: F, t1757: F, t1800: F, t4581: F, t8878: F, t8882: F, t167: F, t10520: F, t22591: F, t8: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23030 = t5192 * t23029;
    let t23031 = t15916 * t23030;
    let t23033 = t8831 * t719;
    let t23034 = t23033 * t1894;
    let t23035 = t1873 * t23034;
    let t23036 = t1869 * t23035;
    let t23038 = t8831 * t642;
    let t23039 = t23038 * t1757;
    let t23040 = t1800 * t23039;
    let t23041 = t1869 * t23040;
    let t23044 = t4581 * t8878;
    let t23045 = t1869 * t23044;
    let t23047 = t4581 * t8882;
    let t23048 = t1869 * t23047;
    let t23050 = 2.0 * t167;
    let t23052 = t22591 * t8 - t10520 - t23050;
    (t23031, t23033, t23034, t23036, t23039, t23041, t23045, t23048, t23052)
}
