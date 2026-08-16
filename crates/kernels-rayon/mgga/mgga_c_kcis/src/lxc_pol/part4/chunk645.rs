//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 645/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk645(t1220: f64, t3551: f64, t2917: f64, t2966: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t2945: f64, t2953: f64, t2961: f64, t2963: f64, t2968: f64, t2972: f64, t2975: f64, t2978: f64) -> (f64, f64, f64, f64) {
    let t3552 = t3551 * t1220;
    let t3557 = 0.68863333333333333333e0_f64 * t2917;
    let t3564 = 0.17365833333333333333e0_f64 * t2966;
    let t3569 = -0.17648625e1_f64 * t2945 + 0.3529725e1_f64 * t2953 + t3557 + 0.34431666666666666666e0_f64 * t2919 - 0.34431666666666666667e0_f64 * t2922 + 0.103295e1_f64 * t2925 - 0.516475e0_f64 * t2928 + 0.31558125e0_f64 * t2961 + 0.6311625e0_f64 * t2963 + t3564 + 0.13892666666666666667e0_f64 * t2968 - 0.34731666666666666667e-1_f64 * t2972 + 0.20839e0_f64 * t2975 - 0.104195e0_f64 * t2978;
    (t3552, t3557, t3564, t3569)
}
