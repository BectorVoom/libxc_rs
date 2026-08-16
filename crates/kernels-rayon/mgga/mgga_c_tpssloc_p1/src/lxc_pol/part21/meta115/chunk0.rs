//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 790/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk790(t2771: f64, t2826: f64, t136: f64, t2776: f64, t908: f64, t2780: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t2800: f64, t2808: f64, t2810: f64, t2816: f64, t2818: f64, t2823: f64, t2824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2827 = t2826 * t2771;
    let t2828 = t136 * t2827;
    let t2830 = t908 * t2776;
    let t2831 = t136 * t2830;
    let t2833 = t908 * t2780;
    let t2834 = t136 * t2833;
    let t2836 = -0.9494625e0_f64 * t2800 + 0.1898925e1_f64 * t2808 + t2810 + 0.19931111111111111111e0_f64 * t2766 - 0.19931111111111111111e0_f64 * t2773 + 0.59793333333333333334e0_f64 * t2778 - 0.29896666666666666667e0_f64 * t2782 + 0.15358125e0_f64 * t2816 + 0.3071625e0_f64 * t2818 + t2823 + 0.10954222222222222222e0_f64 * t2824 - 0.27385555555555555556e-1_f64 * t2828 + 0.16431333333333333333e0_f64 * t2831 - 0.82156666666666666667e-1_f64 * t2834;
    (t2827, t2828, t2830, t2831, t2833, t2834, t2836)
}
