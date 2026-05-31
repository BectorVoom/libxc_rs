//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 639/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk639<F: Float>(t1193: F, t69: F, t3029: F, t608: F, t1941: F, t612: F, t3002: F, t3032: F, t3035: F, t3037: F, t3040: F, t3042: F, t3045: F, t3047: F, t3050: F, t3052: F, t3055: F, t565: F, t584: F) -> (F, F, F) {
    let t3057 = t69 * t1193;
    let t3060 = t608 * t3029;
    let t3062 = t1941 * t1193;
    let t3065 = t612 * t3029;
    let t3067 = t3002 * t584 / F::cast_from(6.0_f64) - t565 * t3029 / F::cast_from(18.0_f64) - t3032 * t584 / F::cast_from(48.0_f64) + t3035 / F::cast_from(240.0_f64) + t3037 * t584 / F::cast_from(640.0_f64) - t3040 / F::cast_from(4480.0_f64) - t3042 * t584 / F::cast_from(11520.0_f64) + t3045 / F::cast_from(103680.0_f64) + t3047 * t584 / F::cast_from(258048.0_f64) - t3050 / F::cast_from(2838528.0_f64) - t3052 * t584 / F::cast_from(6881280.0_f64) + t3055 / F::cast_from(89456640.0_f64) + t3057 * t584 / F::cast_from(0.21233664e9_f64) - t3060 / F::cast_from(0.31850496e10_f64) - t3062 * t584 / F::cast_from(0.74317824e10_f64) + t3065 / F::cast_from(0.1263403008e12_f64);
    (t3057, t3062, t3067)
}
