//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1889/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1889<F: Float>(t1408: F, t1877: F, t1915: F, t22959: F, t23290: F, t25: F, t25013: F, t25015: F, t25021: F, t25024: F, t25028: F, t2522: F, t25354: F, t25358: F, t25366: F, t25372: F, t25375: F, t25377: F, t25381: F, t25385: F, t25392: F, t25397: F, t606: F, t6542: F, t6666: F, t6670: F, t6671: F, t7475: F, t7541: F, t7545: F) -> F {
    let t25398 = F::cast_from(3.0_f64) * t25013 * t25015 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t6666 * t7475 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t25021 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t25024 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t25028 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t7541 * t6542 + t1877 * t25354 * t25 / F::cast_from(2.0_f64) - t1877 * t25358 * t6671 / F::cast_from(2.0_f64) + t1877 * t7541 * t606 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t22959 * t25366 - t1877 * t23290 * t7545 / F::cast_from(2.0_f64) + t25372 * t25375 - t1877 * t6670 * t25377 / F::cast_from(2.0_f64) - t1877 * t6670 * t25381 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t25385 + t1877 * t6666 * t1408 / F::cast_from(2.0_f64) - t1877 * t6670 * t25392 / F::cast_from(2.0_f64) + t25397;
    t25398
}
