//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1235/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1235<F: Float>(t1396: F, t15936: F, t4123: F, t1464: F, t3728: F, t5678: F, t4148: F, t5752: F, t1394: F, t4154: F, t4153: F, t11776: F, t1947: F) -> (F, F, F, F, F, F) {
    let t15937 = t1396 * t15936;
    let t15938 = t4123 * t15937;
    let t15939 = t1464 * t15938;
    let t15941 = t3728 * t5678;
    let t15942 = F::cast_from(0.66327777777777777776e-2_f64) * t15941;
    let t15943 = t5752 * t4148;
    let t15944 = t1394 * t15943;
    let t15946 = t5752 * t4154;
    let t15947 = t4153 * t15946;
    let t15949 = t11776 * t1947;
    (t15939, t15941, t15942, t15944, t15947, t15949)
}
