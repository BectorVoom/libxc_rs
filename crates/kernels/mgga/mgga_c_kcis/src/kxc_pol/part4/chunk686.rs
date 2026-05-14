//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 686/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk686<F: Float>(t4116: F, t4181: F, t589: F, t1502: F, t1505: F, t1555: F, t1504: F, t588: F, t561: F, t143: F, t3951: F, t456: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4182 = t4116 + t4181;
    let t4183 = t4182 * t589;
    let t4184 = t1502 * t1505;
    let t4186 = 2.0 * t4184 * t1555;
    let t4188 = 1.0 / t1504 / t588;
    let t4189 = t561 * t4188;
    let t4190 = t1555 * t1555;
    let t4192 = 2.0 * t4189 * t4190;
    let t4193 = t3951 * t143;
    let t4202 = t562 * t456;
    (t4182, t4183, t4184, t4186, t4188, t4189, t4190, t4192, t4193, t4202)
}
