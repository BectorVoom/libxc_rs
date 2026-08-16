//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1012/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1012<F: Float>(t101226: F, t115027: F, t126177: F, t128076: F, t128080: F, t128086: F, t1408: F, t1877: F, t1914: F, t22960: F, t24191: F, t25: F, t2522: F, t25373: F, t26744: F, t28252: F, t28456: F, t28459: F, t28462: F, t31434: F, t33466: F, t33486: F, t5397: F, t7114: F, t7475: F, t8566: F, t8569: F) -> F {
    let t128093 = t1877 * t115027 * t28456 + F::cast_from(3.0_f64) * t2522 * t8566 * t28252 + t1877 * t33466 * t1408 - t1877 * t26744 * t33486 - t1877 * t7114 * t5397 * t1914 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t33466 * t7475 - t1877 * t31434 * t28462 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t24191 * t126177 + t1877 * t8566 * t5397 / F::cast_from(2.0_f64) + t1877 * t128076 * t25 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t24191 * t25373 * t128080 - t1877 * t31434 * t28459 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24191 * t22960 * t128086 - t1877 * t101226 * t8569 / F::cast_from(2.0_f64);
    t128093
}
