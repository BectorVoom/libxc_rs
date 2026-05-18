//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1324/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1324<F: Float>(t27870: F, t2822: F, t1087: F, t303: F, t5013: F, t15573: F, t27914: F, t2173: F, t26736: F, t26748: F, t27775: F, t27780: F, t27812: F, t27919: F, t7687: F, t8030: F, t93366: F, t93690: F, t93694: F, t95629: F, t95898: F) -> (F, F, F, F) {
    let t96345 = t2822 * t27870;
    let t96354 = t303 * t1087 * t5013;
    let t96356 = t15573 * t27914;
    let t96358 = F::new(0.46336805555555555556e-3) * t2173 * t96356;
    let t96369 = -F::new(0.44218518518518518517e-2) * t96345 - F::new(0.12356481481481481482e-2) * t93690 + F::new(0.69505208333333333333e-3) * t8030 * t26736 + F::new(0.69505208333333333333e-3) * t2173 * t95898 - F::new(0.12356481481481481482e-2) * t93694 - F::new(0.88437037037037037034e-2) * t96354 + t96358 + F::new(0.13901041666666666667e-2) * t7687 * t27919 - F::new(0.27802083333333333334e-2) * t26748 * t27775 - F::new(0.13901041666666666667e-2) * t26748 * t27780 - F::new(0.18550940104166666667e-3) * t93366 * t27780 - F::new(0.185671721767578125e-4) * t27812 * t95629;
    (t96345, t96354, t96356, t96369)
}
