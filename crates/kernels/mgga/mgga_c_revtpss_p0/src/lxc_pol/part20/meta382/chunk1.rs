//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1389/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1389<F: Float>(t40861: F, t802: F, t10899: F, t794: F, t10902: F, t159: F, t216: F, t2475: F, t10764: F, t10770: F, t10771: F, t10785: F, t10786: F, t124: F, t2646: F, t2724: F, t2745: F, t2747: F, t2754: F, t39476: F, t40232: F, t40446: F, t40569: F, t40816: F, t40822: F, t40824: F, t40836: F, t40838: F, t40840: F, t40850: F, t40851: F, t40855: F, t4362: F, t4364: F, t4366: F, t799: F, t800: F) -> F {
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40865 = t40864 * t10902;
    let t40868 = t216 * t159 * t2475;
    let t40873 = F::cast_from(0.51448821741683684366e-2_f64) * t2745 * t2747 * t10764 * t2754 - F::cast_from(0.48018900292238105408e-1_f64) * t40816 + F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t4364 * t40569 * t4366 + F::cast_from(0.96037800584476210818e-1_f64) * t40822 - F::cast_from(0.24009450146119052704e-1_f64) * t40824 - F::cast_from(0.20579528696673473747e-1_f64) * t4362 * t2747 * t40446 * t10786 + F::cast_from(0.51448821741683684368e-1_f64) * t4362 * t10770 * t10771 * t2724 - F::cast_from(0.12196800674228478774e-3_f64) * t40836 - F::new(35.0) / F::new(36.0) * t40838 + F::new(7.0) / F::new(36.0) * t40840 - t799 * t800 * t124 * t39476 / F::new(48.0) - t40850 + F::cast_from(0.91464571985215438873e-2_f64) * t40851 + F::cast_from(0.6098400337114239387e-3_f64) * t40855 - F::cast_from(0.12862205435420921092e-2_f64) * t2745 * t4364 * t10785 * t2646 + F::new(455.0) / F::new(162.0) * t40862 + F::new(7.0) / F::new(3.0) * t40865 + F::new(5.0) / F::new(4.0) * t40868 * t800 * t124 * t40232;
    t40873
}
