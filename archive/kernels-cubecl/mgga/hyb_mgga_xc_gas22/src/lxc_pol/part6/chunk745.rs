//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 745/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk745<F: Float>(t1954: F, t3881: F, t1975: F, t1967: F, t3876: F, t623: F, t627: F, t74: F, t79: F, t81: F, t82: F, t1211: F, t1223: F, t1959: F, t618: F, t72: F, t85: F) -> (F, F, F, F) {
    let t3882 = t1954 * t3881;
    let t3898 = t1975 * t3881;
    let t3909 = -F::cast_from(2.0_f64) * t1967 * t3881 * t81 + t623 * t3876 * t81 / F::cast_from(2.0_f64) + t3898 * t81 / F::cast_from(4.0_f64) - F::cast_from(4.0_f64) * t3881 * t82 - t79 * t3881 * t81 - F::cast_from(4.0_f64) * t627 * t3876 - t74 * t3876 * t81;
    let t3912 = -t3882 * t81 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t1959 * t3881 - t618 * t3876 + F::cast_from(2.0_f64) * t3876 * t85 + F::cast_from(4.0_f64) * t1211 * t1223 + F::cast_from(2.0_f64) * t72 * t3909;
    (t3882, t3898, t3909, t3912)
}
