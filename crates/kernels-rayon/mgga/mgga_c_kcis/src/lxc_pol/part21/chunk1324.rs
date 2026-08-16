//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1324/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1324(t27870: f64, t2822: f64, t1087: f64, t303: f64, t5013: f64, t15573: f64, t27914: f64, t2173: f64, t26736: f64, t26748: f64, t27775: f64, t27780: f64, t27812: f64, t27919: f64, t7687: f64, t8030: f64, t93366: f64, t93690: f64, t93694: f64, t95629: f64, t95898: f64) -> (f64, f64, f64, f64) {
    let t96345 = t2822 * t27870;
    let t96354 = t303 * t1087 * t5013;
    let t96356 = t15573 * t27914;
    let t96358 = 0.46336805555555555556e-3_f64 * t2173 * t96356;
    let t96369 = -0.44218518518518518517e-2_f64 * t96345 - 0.12356481481481481482e-2_f64 * t93690 + 0.69505208333333333333e-3_f64 * t8030 * t26736 + 0.69505208333333333333e-3_f64 * t2173 * t95898 - 0.12356481481481481482e-2_f64 * t93694 - 0.88437037037037037034e-2_f64 * t96354 + t96358 + 0.13901041666666666667e-2_f64 * t7687 * t27919 - 0.27802083333333333334e-2_f64 * t26748 * t27775 - 0.13901041666666666667e-2_f64 * t26748 * t27780 - 0.18550940104166666667e-3_f64 * t93366 * t27780 - 0.185671721767578125e-4_f64 * t27812 * t95629;
    (t96345, t96354, t96356, t96369)
}
