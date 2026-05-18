//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1037/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1037<F: Float>(t1396: F, t4161: F, t12240: F, t4142: F, t5770: F, t1017: F, t541: F, t86: F, t3728: F, t5882: F, t5678: F, t1494: F, t2001: F) -> (F, F, F, F, F, F, F, F) {
    let t15878 = t4161 * t1396;
    let t15887 = t12240 * t1396;
    let t15896 = t4142 * t5770;
    let t15909 = t86 * t1017 * t541;
    let t15934 = t3728 * t5882;
    let t15941 = t3728 * t5678;
    let t15942 = F::new(0.66327777777777777776e-2) * t15941;
    let t15955 = t1494 * t2001;
    (t15878, t15887, t15896, t15909, t15934, t15941, t15942, t15955)
}
