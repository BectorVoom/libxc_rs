//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 544/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk544<F: Float>(t2771: F, t2826: F, t136: F, t2776: F, t908: F, t2780: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2810: F, t2816: F, t2818: F, t2823: F, t2824: F) -> (F, F, F, F, F, F, F) {
    let t2827 = t2826 * t2771;
    let t2828 = t136 * t2827;
    let t2830 = t908 * t2776;
    let t2831 = t136 * t2830;
    let t2833 = t908 * t2780;
    let t2834 = t136 * t2833;
    let t2836 = -F::cast_from(0.9494625e0_f64) * t2800 + F::cast_from(0.1898925e1_f64) * t2808 + t2810 + F::cast_from(0.19931111111111111111e0_f64) * t2766 - F::cast_from(0.19931111111111111111e0_f64) * t2773 + F::cast_from(0.59793333333333333334e0_f64) * t2778 - F::cast_from(0.29896666666666666667e0_f64) * t2782 + F::cast_from(0.15358125e0_f64) * t2816 + F::cast_from(0.3071625e0_f64) * t2818 + t2823 + F::cast_from(0.10954222222222222222e0_f64) * t2824 - F::cast_from(0.27385555555555555556e-1_f64) * t2828 + F::cast_from(0.16431333333333333333e0_f64) * t2831 - F::cast_from(0.82156666666666666667e-1_f64) * t2834;
    (t2827, t2828, t2830, t2831, t2833, t2834, t2836)
}
