//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 794/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk794(t13948: f64, t4715: f64, t13712: f64, t13714: f64, t13908: f64, t1728: f64, t3054: f64, t1068: f64, t1717: f64, t1750: f64, t3245: f64, t3209: f64, t3218: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13949 = t13948 * t4715;
    let t13962 = 0.41203703703703703704e-2_f64 * t13712;
    let t13963 = 0.12361111111111111111e-1_f64 * t13714;
    let t14001 = 0.22076e0_f64 * t13908;
    let t14015 = 0.13418888888888888889e0_f64 * t13712;
    let t14053 = t3054 * t1728;
    let t14055 = t1068 * t1717;
    let t14065 = t3245 * t1750;
    let t14067 = t3209 * t3218;
    (t13949, t13962, t13963, t14001, t14015, t14053, t14055, t14065, t14067)
}
