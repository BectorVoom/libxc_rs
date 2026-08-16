//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1065/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1065(t39977: f64, t39997: f64, t2604: f64, t39975: f64, t39979: f64, t39985: f64, t39994: f64, t40002: f64, t40007: f64, t40012: f64, t40015: f64, t40018: f64, t40021: f64, t40024: f64, t40027: f64, t5144: f64, t5199: f64, t5267: f64, t699: f64, t739: f64, t8264: f64, t884: f64, t903: f64, t9321: f64) -> f64 {
    let t43207 = 0.39726959900411316772e-4_f64 * t39977;
    let t43211 = 0.3193131120497015617e0_f64 * t39997;
    let t43231 = -0.1702583995731913576e-4_f64 * t39975 - t43207 + 0.5107751987195740728e-4_f64 * t39979 + 0.5107751987195740728e-4_f64 * t39985 + 0.17961362552795712846e0_f64 * t39994 + t43211 + 0.212822999466489197e-4_f64 * t40002 - 0.5107751987195740728e-4_f64 * t40007 + 0.23948483403727617128e0_f64 * t739 * t8264 * t5144 - 0.23948483403727617128e0_f64 * t884 * t8264 * t5267 + 0.17961362552795712846e0_f64 * t903 * t699 * t5199 + 0.23948483403727617128e0_f64 * t2604 * t9321 - 0.32729593985094410076e0_f64 * t40012 - 0.81823984962736025192e-1_f64 * t40015 + 0.16364796992547205038e0_f64 * t40018 + 0.11974241701863808564e0_f64 * t40021 - 0.17961362552795712846e0_f64 * t40024 - 0.11974241701863808564e0_f64 * t40027;
    t43231
}
