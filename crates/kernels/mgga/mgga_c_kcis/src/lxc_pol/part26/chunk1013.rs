//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1013/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1013<F: Float>(t1572: F, t22979: F, t12761: F, t1564: F, t21311: F, t21356: F, t21359: F, t21362: F, t21365: F, t21369: F, t21372: F, t21376: F, t21402: F, t4326: F, t6075: F, t6098: F, t7444: F, t7460: F) -> F {
    let t22980 = t22979 * t1572;
    let t22983 = -t21356 + t21359 + t21362 + t21365 - t21369 - t21372 - t21376 - F::new(0.19751789702565206229e-1) * t21311 + t21402 + F::new(2.0) * t6075 * t6098 - F::new(2.0) * t12761 * t7444 + F::new(1.0) * t4326 * t7460 + F::new(1.0) * t1564 * t22980;
    t22983
}
