//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 579/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk579(t3005: f64, t3006: f64, t971: f64, t2917: f64, t2966: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t2945: f64, t2953: f64, t2961: f64, t2963: f64, t2968: f64, t2972: f64, t2975: f64, t2978: f64) -> (f64, f64, f64, f64) {
    let t3008 = t3005 * t3006 * t971;
    let t3013 = 0.40256666666666666667e0_f64 * t2917;
    let t3020 = 0.137975e0_f64 * t2966;
    let t3025 = -0.1294625e1_f64 * t2945 + 0.258925e1_f64 * t2953 + t3013 + 0.20128333333333333334e0_f64 * t2919 - 0.20128333333333333333e0_f64 * t2922 + 0.60385e0_f64 * t2925 - 0.301925e0_f64 * t2928 + 0.82524375e-1_f64 * t2961 + 0.16504875e0_f64 * t2963 + t3020 + 0.11038e0_f64 * t2968 - 0.27595e-1_f64 * t2972 + 0.16557e0_f64 * t2975 - 0.82785e-1_f64 * t2978;
    (t3008, t3013, t3020, t3025)
}
