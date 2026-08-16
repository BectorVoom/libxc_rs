//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1389/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1389(t40861: f64, t802: f64, t10899: f64, t794: f64, t10902: f64, t159: f64, t216: f64, t2475: f64, t10764: f64, t10770: f64, t10771: f64, t10785: f64, t10786: f64, t124: f64, t2646: f64, t2724: f64, t2745: f64, t2747: f64, t2754: f64, t39476: f64, t40232: f64, t40446: f64, t40569: f64, t40816: f64, t40822: f64, t40824: f64, t40836: f64, t40838: f64, t40840: f64, t40850: f64, t40851: f64, t40855: f64, t4362: f64, t4364: f64, t4366: f64, t799: f64, t800: f64) -> f64 {
    let t40862 = t40861 * t802;
    let t40864 = t794 * t10899;
    let t40865 = t40864 * t10902;
    let t40868 = t216 * t159 * t2475;
    let t40873 = 0.51448821741683684366e-2_f64 * t2745 * t2747 * t10764 * t2754 - 0.48018900292238105408e-1_f64 * t40816 + 0.17149607247227894789e-2_f64 * t4362 * t4364 * t40569 * t4366 + 0.96037800584476210818e-1_f64 * t40822 - 0.24009450146119052704e-1_f64 * t40824 - 0.20579528696673473747e-1_f64 * t4362 * t2747 * t40446 * t10786 + 0.51448821741683684368e-1_f64 * t4362 * t10770 * t10771 * t2724 - 0.12196800674228478774e-3_f64 * t40836 - 35.0_f64 / 36.0_f64 * t40838 + 7.0_f64 / 36.0_f64 * t40840 - t799 * t800 * t124 * t39476 / 48.0_f64 - t40850 + 0.91464571985215438873e-2_f64 * t40851 + 0.6098400337114239387e-3_f64 * t40855 - 0.12862205435420921092e-2_f64 * t2745 * t4364 * t10785 * t2646 + 455.0_f64 / 162.0_f64 * t40862 + 7.0_f64 / 3.0_f64 * t40865 + 5.0_f64 / 4.0_f64 * t40868 * t800 * t124 * t40232;
    t40873
}
