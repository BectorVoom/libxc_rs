//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1187/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1187<F: Float>(t19609: F, t5176: F, t14849: F, t1195: F, t6716: F, t3442: F, t1176: F, t6682: F, t1133: F, t6491: F, t5077: F, t3337: F) -> (F, F, F, F, F) {
    let t19882 = t5176 * t19609;
    let t19883 = t14849 * t19882;
    let t19885 = t1195 * t6716;
    let t19886 = t3442 * t19885;
    let t19888 = t6682 * t1176;
    let t19890 = t6491 * t1133;
    let t19891 = t5077 * t19890;
    let t19892 = t3337 * t19891;
    (t19883, t19886, t19888, t19890, t19892)
}
