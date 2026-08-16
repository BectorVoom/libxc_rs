//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 438/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk438<F: Float>(t1668: F, t1682: F, t1685: F, t1694: F, t45: F, t960: F) -> F {
    let t1697 = -t1668 + t1682 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t1685 - F::cast_from(0.58482233974552040708e0_f64) * t960 * t1694;
    t1697
}
