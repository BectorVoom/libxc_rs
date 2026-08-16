//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 843/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk843<F: Float>(t231: F, t245: F, t6527: F, t228: F, t1792: F, t239: F) -> (F, F, F, F, F) {
    let t6585 = F::cast_from(1.0_f64) / t231 / t245 / F::cast_from(4.0_f64);
    let t6592 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6527;
    let t6597 = F::cast_from(0.93011851851851851854e0_f64) * t6527;
    let t6601 = F::cast_from(1.0_f64)/pow_3_2::<F>(t228);
    let t6610 = F::cast_from(1.0_f64) / t239 / t1792;
    (t6585, t6592, t6597, t6601, t6610)
}
