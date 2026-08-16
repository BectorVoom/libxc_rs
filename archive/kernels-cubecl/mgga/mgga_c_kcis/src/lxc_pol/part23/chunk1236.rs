//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1236/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1236<F: Float>(t1464: F, t15910: F, t3722: F, t7923: F, t28351: F, t3715: F, t51121: F, t20905: F, t27387: F, t4136: F, t16686: F, t4153: F) -> (F, F, F, F) {
    let t98171 = t1464 * t7923 * t15910 * t3722;
    let t98174 = t28351 * t51121 * t3715;
    let t98179 = t1464 * t27387 * t20905 * t4136;
    let t98188 = t4153 * t7923 * t16686;
    (t98171, t98174, t98179, t98188)
}
