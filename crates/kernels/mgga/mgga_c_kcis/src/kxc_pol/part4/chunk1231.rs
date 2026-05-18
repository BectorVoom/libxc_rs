//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1231/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1231<F: Float>(t15887: F, t15888: F, t4160: F, t1397: F, t15808: F, t1394: F, t4142: F, t5770: F, t2001: F, t3722: F, t1495: F, t1468: F) -> (F, F, F, F, F) {
    let t15889 = t15887 * t15888;
    let t15890 = t4160 * t15889;
    let t15893 = t15808 * t1397;
    let t15894 = t1394 * t15893;
    let t15896 = t4142 * t5770;
    let t15898 = t2001 * t3722;
    let t15899 = t1495 * t15898;
    let t15900 = t1468 * t15899;
    (t15890, t15894, t15896, t15898, t15900)
}
