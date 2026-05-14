//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 487/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk487<F: Float>(t1237: F, t4037: F, t4007: F, t4011: F, t4015: F, t4018: F, t4021: F, t1235: F, t344: F, t1242: F, t313: F, t353: F, t964: F, t1163: F, t1248: F, t3979: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4038 = t1237 * t1237;
    let t4039 = t4037 * t4038;
    let t4041 = 4.0 / 9.0 * t4007;
    let t4046 = t4041 + 2.0 / 9.0 * t4011 - 2.0 / 9.0 * t4015 + 2.0 / 3.0 * t4018 - t4021 / 3.0;
    let t4047 = t1235 * t4046;
    let t4049 = 0.39862222222222222223e0 * t4007;
    let t4054 = 1.0/f64::sqrt(t344);
    let t4055 = t4054 * t4038;
    let t4057 = t1242 * t4046;
    let t4060 = t353 * t964 * t313;
    let t4061 = 0.27385555555555555555e0 * t4060;
    let t4063 = t1248 * t3979 * t1163;
    (t4038, t4039, t4046, t4047, t4049, t4054, t4055, t4057, t4060, t4061, t4063)
}
