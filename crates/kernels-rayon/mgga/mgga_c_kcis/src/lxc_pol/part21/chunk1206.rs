//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1206/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1206(t26490: f64, t7642: f64, t26459: f64, t7647: f64, t26494: f64, t26508: f64, t26477: f64, t36936: f64, t695: f64, t92066: f64, t92068: f64, t92070: f64, t92072: f64, t92074: f64, t92076: f64, t92078: f64, t92080: f64, t92082: f64, t92086: f64, t92089: f64, t92091: f64, t92093: f64) -> f64 {
    let t92095 = t7642 * t26490;
    let t92097 = t26459 * t7647;
    let t92099 = t26508 * t26494;
    let t92102 = t36936 * t695 * t26477;
    let t92104 = -0.12985658072916666667e-2_f64 * t92066 + 0.208515625e-2_f64 * t92068 - 0.97307291666666666666e-2_f64 * t92070 - 0.97307291666666666666e-2_f64 * t92072 - 0.41703125000000000001e-2_f64 * t92074 + 0.10203017057291666667e-2_f64 * t92076 - 0.41703125000000000001e-2_f64 * t92078 + 0.69505208333333333333e-3_f64 * t92080 - 0.48653645833333333332e-2_f64 * t92082 + 0.69505208333333333333e-3_f64 * t92086 + 0.1299702052373046875e-3_f64 * t92089 + 0.16217881944444444444e-1_f64 * t92091 + 0.16217881944444444444e-1_f64 * t92093 - 0.48653645833333333332e-2_f64 * t92095 + 0.208515625e-2_f64 * t92097 + 0.2782641015625e-3_f64 * t92099 - 0.55701516530273437501e-4_f64 * t92102;
    t92104
}
