//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 929/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk929<F: Float>(t13908: F, t4708: F, t659: F, t13714: F, t1676: F, t2331: F, t22: F, t4864: F, t4715: F, t13712: F, t1728: F, t3054: F, t1068: F, t1717: F, t1750: F, t3245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13909 = 0.21908444444444444444e0 * t13908;
    let t13912 = t659 * t4708;
    let t13939 = 0.39862222222222222222e0 * t13714;
    let t13945 = t2331 * t1676;
    let t13948 = t22 * t4864;
    let t13949 = t13948 * t4715;
    let t13962 = 0.41203703703703703704e-2 * t13712;
    let t13963 = 0.12361111111111111111e-1 * t13714;
    let t14001 = 0.22076e0 * t13908;
    let t14015 = 0.13418888888888888889e0 * t13712;
    let t14053 = t3054 * t1728;
    let t14055 = t1068 * t1717;
    let t14065 = t3245 * t1750;
    (t13909, t13912, t13939, t13945, t13948, t13949, t13962, t13963, t14001, t14015, t14053, t14055, t14065)
}
