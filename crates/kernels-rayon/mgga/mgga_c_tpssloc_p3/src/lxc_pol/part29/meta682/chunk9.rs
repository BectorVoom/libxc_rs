//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2313/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2313(t2132: f64, t24746: f64, t95382: f64, t24655: f64, t24664: f64, t24670: f64, t24685: f64, t27629: f64, t27636: f64, t27638: f64, t27642: f64, t27692: f64, t3032: f64, t3503: f64, t3507: f64, t3566: f64, t475: f64, t488: f64, t4954: f64, t5011: f64, t7331: f64, t8040: f64, t8048: f64, t86199: f64, t86330: f64, t95370: f64, t95384: f64, t95387: f64, t95396: f64) -> f64 {
    let t95404 = 0.20186378047070195428e-3_f64 * t2132 * t95382 * t24746;
    let t95407 = t95370 - t3566 * t8048 * t488 / 288.0_f64 - 0.20186378047070195428e-3_f64 * t24685 * t27692 + 0.40372756094140390856e-3_f64 * t27636 * t3503 * t5011 * t27638 - 0.10093189023535097714e-3_f64 * t27629 * t24655 + 0.20186378047070195428e-3_f64 * t95384 * t7331 - 0.20186378047070195428e-3_f64 * t95387 * t24664 + 0.10093189023535097714e-3_f64 * t95387 * t24670 - 0.10093189023535097714e-3_f64 * t86199 * t8040 + 0.10093189023535097714e-3_f64 * t95396 * t27642 * t3032 * t3507 * t475 + t95404 - t86330 * t4954 / 1152.0_f64;
    t95407
}
