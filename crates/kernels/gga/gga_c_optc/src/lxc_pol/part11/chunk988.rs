//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 988/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk988<F: Float>(t1179: F, t12727: F, t12729: F, t15874: F, t18055: F, t18059: F, t18062: F, t18066: F, t18069: F, t18072: F, t18076: F, t3244: F, t4457: F, t4464: F, t9093: F, t9102: F, t9116: F, t9122: F) -> F {
    let t18080 = -F::cast_from(0.19318136643975017455e-1_f64) * t12727 - F::cast_from(0.33587136305576131526e-2_f64) * t12729 + t9093 - F::cast_from(0.13186481011862155443e4_f64) * t4464 * t18055 + F::cast_from(0.34014423178468276541e6_f64) * t9116 * t18059 - F::cast_from(0.34014423178468276541e6_f64) * t9122 * t18062 + F::cast_from(0.26372962023724310886e4_f64) * t4457 * t18066 + F::cast_from(0.56690705297447127569e5_f64) * t9102 * t18069 + F::cast_from(0.15146801702008125515e1_f64) * t3244 * t18072 + F::cast_from(0.25190352229182098644e-1_f64) * t1179 * t18076 + F::cast_from(0.75734008510040627575e0_f64) * t15874;
    t18080
}
