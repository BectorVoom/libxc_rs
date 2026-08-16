//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 484/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk484(t2630: f64, t313: f64, t1035: f64, t829: f64, t1045: f64, t2635: f64, t312: f64) -> (f64, f64, f64, f64, f64) {
    let t3062 = t313 * t2630;
    let t3065 = t1035 * t829;
    let t3066 = t3065 * t1045;
    let t3069 = t313 * t2635;
    let t3072 = t312 * t312;
    let t3073 = 1.0_f64 / t3072;
    (t3062, t3066, t3069, t3072, t3073)
}
