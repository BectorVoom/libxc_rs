//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 925/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk925<F: Float>(t11409: F, t11746: F, t16046: F, t16050: F, t16052: F, t16523: F, t21268: F, t21273: F, t21275: F, t21278: F, t21581: F, t11727: F, t11730: F, t1319: F, t1410: F, t16500: F, t16503: F, t1897: F, t21267: F, t21537: F, t21542: F, t21551: F, t21558: F, t3821: F, t3824: F, t456: F, t5481: F, t5503: F, t5510: F, t6957: F, t6964: F) -> F {
    let t21582 = F::cast_from(0.14865e-1_f64) * t21273 - F::cast_from(0.1982e-1_f64) * t21275 - F::cast_from(0.991e-2_f64) * t21278 + F::cast_from(0.1982e-1_f64) * t21268 - t11746 - F::cast_from(0.18344444444444444444e-2_f64) * t11409 - F::cast_from(0.36688888888888888888e-2_f64) * t16046 + t16523 - F::cast_from(0.55033333333333333332e-2_f64) * t16050 - F::cast_from(0.55033333333333333332e-2_f64) * t16052 + t21581;
    let t21585 = F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t11727 * t21537 - t11730 * t6957 / F::cast_from(8.0_f64) - t3821 * t21542 / F::cast_from(4.0_f64) - t16500 * t5503 / F::cast_from(4.0_f64) + t16503 * t1897 / F::cast_from(2.0_f64) + t5510 * t5481 / F::cast_from(2.0_f64) - t3821 * t21551 / F::cast_from(8.0_f64) + t3824 * t6964 / F::cast_from(4.0_f64) + t1410 * t21267 / F::cast_from(4.0_f64) + t21558 * t1319 / F::cast_from(4.0_f64) + t456 * t21582 / F::cast_from(2.0_f64);
    t21585
}
