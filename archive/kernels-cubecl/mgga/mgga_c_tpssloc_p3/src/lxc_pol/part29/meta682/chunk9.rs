//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2313/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313<F: Float>(t2132: F, t24746: F, t95382: F, t24655: F, t24664: F, t24670: F, t24685: F, t27629: F, t27636: F, t27638: F, t27642: F, t27692: F, t3032: F, t3503: F, t3507: F, t3566: F, t475: F, t488: F, t4954: F, t5011: F, t7331: F, t8040: F, t8048: F, t86199: F, t86330: F, t95370: F, t95384: F, t95387: F, t95396: F) -> F {
    let t95404 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t95382 * t24746;
    let t95407 = t95370 - t3566 * t8048 * t488 / F::cast_from(288.0_f64) - F::cast_from(0.20186378047070195428e-3_f64) * t24685 * t27692 + F::cast_from(0.40372756094140390856e-3_f64) * t27636 * t3503 * t5011 * t27638 - F::cast_from(0.10093189023535097714e-3_f64) * t27629 * t24655 + F::cast_from(0.20186378047070195428e-3_f64) * t95384 * t7331 - F::cast_from(0.20186378047070195428e-3_f64) * t95387 * t24664 + F::cast_from(0.10093189023535097714e-3_f64) * t95387 * t24670 - F::cast_from(0.10093189023535097714e-3_f64) * t86199 * t8040 + F::cast_from(0.10093189023535097714e-3_f64) * t95396 * t27642 * t3032 * t3507 * t475 + t95404 - t86330 * t4954 / F::cast_from(1152.0_f64);
    t95407
}
