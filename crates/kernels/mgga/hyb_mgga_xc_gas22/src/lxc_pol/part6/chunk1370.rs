//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1370/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1370<F: Float>(t21390: F, t21393: F, t21396: F, t21425: F, t21427: F, t21430: F, t21433: F, t25214: F, t25217: F, t25220: F, t29819: F, t29818: F, t957: F) -> (F, F) {
    let t29821 = t21390 - F::cast_from(0.18786444444444444445e1_f64) * t21393 + F::cast_from(0.40256666666666666667e0_f64) * t21396 + t21425 + F::cast_from(0.27595e0_f64) * t21430 - F::cast_from(0.14717333333333333333e1_f64) * t21427 + F::cast_from(0.27595e0_f64) * t21433 - F::cast_from(0.18786444444444444444e1_f64) * t25214 + F::cast_from(0.16102666666666666667e1_f64) * t25217 - F::cast_from(0.60385e0_f64) * t25220 + F::cast_from(0.258925e1_f64) * t29819;
    let t29822 = t957 * t29818;
    (t29821, t29822)
}
