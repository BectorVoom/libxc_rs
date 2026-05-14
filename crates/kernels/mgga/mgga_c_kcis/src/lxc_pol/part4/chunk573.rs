//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 573/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk573<F: Float>(t304: F, t3168: F, t355: F, t360: F, t303: F, t1135: F, t2861: F, t1010: F, t2812: F, t2818: F, t2823: F, t2827: F, t2832: F, t2836: F, t2848: F, t2853: F, t2858: F, t2862: F, t3046: F, t3049: F, t3052: F, t979: F) -> (F, F, F, F) {
    let t3169 = t304 * t3168;
    let t3170 = t3169 * t355;
    let t3171 = t3170 * t360;
    let t3172 = t303 * t3171;
    let t3174 = t2861 * t1135;
    let t3176 = 0.66725e-1 * t979 * t2812 - 0.33163888888888888888e-2 * t2818 + 0.22109259259259259258e-2 * t2823 + 0.33163888888888888888e-2 * t2827 + 0.16581944444444444444e-2 * t2832 + 0.890445125e-2 * t2836 * t2812 + 0.27636574074074074073e-2 * t2848 - 0.33163888888888888888e-2 * t2853 - 0.88437037037037037034e-2 * t2858 - 0.33163888888888888888e-2 * t2862 - 0.66725e-1 * t979 * t3046 - 0.13345e0 * t3049 * t1010 + 0.33163888888888888888e-2 * t3052 + 0.24872916666666666666e-2 * t3172 + 0.22109259259259259258e-2 * t3174;
    (t3171, t3172, t3174, t3176)
}
