//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 627/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk627<F: Float>(t1494: F, t531: F, t1502: F, t1505: F, t1504: F, t588: F, t561: F, t456: F, t562: F) -> (F, F, F, F, F) {
    let t4171 = t1494 * t531;
    let t4184 = t1502 * t1505;
    let t4188 = F::cast_from(1.0_f64) / t1504 / t588;
    let t4189 = t561 * t4188;
    let t4202 = t562 * t456;
    (t4171, t4184, t4188, t4189, t4202)
}
