//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 596/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk596(t304: f64, t3168: f64, t355: f64, t360: f64, t303: f64, t1135: f64, t2861: f64, t1010: f64, t2812: f64, t2818: f64, t2823: f64, t2827: f64, t2832: f64, t2836: f64, t2848: f64, t2853: f64, t2858: f64, t2862: f64, t3046: f64, t3049: f64, t3052: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t3169 = t304 * t3168;
    let t3170 = t3169 * t355;
    let t3171 = t3170 * t360;
    let t3172 = t303 * t3171;
    let t3174 = t2861 * t1135;
    let t3176 = 0.66725e-1_f64 * t979 * t2812 - 0.33163888888888888888e-2_f64 * t2818 + 0.22109259259259259258e-2_f64 * t2823 + 0.33163888888888888888e-2_f64 * t2827 + 0.16581944444444444444e-2_f64 * t2832 + 0.890445125e-2_f64 * t2836 * t2812 + 0.27636574074074074073e-2_f64 * t2848 - 0.33163888888888888888e-2_f64 * t2853 - 0.88437037037037037034e-2_f64 * t2858 - 0.33163888888888888888e-2_f64 * t2862 - 0.66725e-1_f64 * t979 * t3046 - 0.13345e0_f64 * t3049 * t1010 + 0.33163888888888888888e-2_f64 * t3052 + 0.24872916666666666666e-2_f64 * t3172 + 0.22109259259259259258e-2_f64 * t3174;
    (t3171, t3172, t3174, t3176)
}
