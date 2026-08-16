//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2325/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325(t27710: f64, t3: f64, t24684: f64, t15608: f64, t24741: f64, t11716: f64, t1210: f64, t14744: f64, t1714: f64, t1734: f64, t2121: f64, t2132: f64, t24699: f64, t27636: f64, t27637: f64, t27642: f64, t27644: f64, t27704: f64, t3448: f64, t3507: f64, t475: f64, t4950: f64, t5011: f64, t6729: f64, t7321: f64, t7331: f64, t8040: f64, t85827: f64, t85966: f64, t85972: f64, t86194: f64, t86330: f64, t86357: f64, t95396: f64) -> f64 {
    let t95648 = t27710 * t3;
    let t95649 = t95648 * t24684;
    let t95662 = t24741 * t15608 / 1728.0_f64;
    let t95672 = 0.60559134141210586284e-3_f64 * t95396 * t11716 * t1734 * t85966 * t3507 - 0.60559134141210586284e-3_f64 * t95396 * t27637 * t85972 * t3507 - 0.10093189023535097714e-3_f64 * t27636 * t27642 * t85827 * t475 + 0.16149102437656156342e-2_f64 * t95649 * t7331 - 0.20186378047070195428e-3_f64 * t27636 * t1210 * t5011 * t27644 + 0.20186378047070195428e-3_f64 * t86194 * t8040 - 0.10093189023535097714e-3_f64 * t86357 - t86330 * t4950 / 1152.0_f64 - t95662 - t2121 * t3448 * t14744 / 48.0_f64 - 0.10093189023535097714e-3_f64 * t27704 * t24699 + 0.20186378047070195428e-3_f64 * t2132 * t6729 * t1714 * t7321;
    t95672
}
