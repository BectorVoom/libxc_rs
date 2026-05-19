//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 798/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk798<F: Float>(t312: F, t4368: F, t4371: F, t4374: F, t1454: F, t1455: F, t1459: F, t1479: F, t1480: F, t1486: F, t3951: F, t3957: F, t3963: F, t3969: F, t3974: F, t398: F, t401: F, t414: F, t415: F, t423: F, t431: F, t4356: F, t4363: F, t4383: F, t4389: F, t4390: F, t4394: F, t4397: F, t4400: F) -> F {
    let t4403 = t312 * t4368;
    let t4404 = t4371 * t4374;
    let t4407 = F::cast_from(0.28999131295963805491e1_f64) * t431 * t414 * t4356 * t423 - F::cast_from(0.2854310864347144482e1_f64) * t431 * t1479 * t4363 * t1486 + F::cast_from(0.70082276486377300979e0_f64) * t431 * t4368 * t4371 * t4374 + F::cast_from(0.458714896073149408e1_f64) * t398 * t3951 * t401 - F::new(40.0) / F::new(9.0) * t1459 * t3969 + F::cast_from(0.28999131295963805491e1_f64) * t415 * t4383 - F::cast_from(0.52822214337494074078e1_f64) * t1454 * t3957 * t1455 + F::cast_from(0.14685052460713464727e1_f64) * t4389 * t3963 * t4390 + F::new(50.0) / F::new(9.0) * t4394 * t3974 + F::new(50.0) / F::new(9.0) * t4397 * t3974 - F::cast_from(0.2854310864347144482e1_f64) * t1480 * t4400 + F::cast_from(0.70082276486377300979e0_f64) * t4403 * t4404;
    t4407
}
