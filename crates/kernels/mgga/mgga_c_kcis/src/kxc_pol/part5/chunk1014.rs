//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1014/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1014<F: Float>(t13948: F, t4715: F, t13712: F, t13714: F, t13908: F, t1728: F, t3054: F, t1068: F, t1717: F, t1750: F, t3245: F, t3209: F, t3218: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13949 = t13948 * t4715;
    let t13962 = F::cast_from(0.41203703703703703704e-2_f64) * t13712;
    let t13963 = F::cast_from(0.12361111111111111111e-1_f64) * t13714;
    let t14001 = F::new(0.22076e0) * t13908;
    let t14015 = F::cast_from(0.13418888888888888889e0_f64) * t13712;
    let t14053 = t3054 * t1728;
    let t14055 = t1068 * t1717;
    let t14065 = t3245 * t1750;
    let t14067 = t3209 * t3218;
    (t13949, t13962, t13963, t14001, t14015, t14053, t14055, t14065, t14067)
}
