//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1065/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1065<F: Float>(t39977: F, t39997: F, t2604: F, t39975: F, t39979: F, t39985: F, t39994: F, t40002: F, t40007: F, t40012: F, t40015: F, t40018: F, t40021: F, t40024: F, t40027: F, t5144: F, t5199: F, t5267: F, t699: F, t739: F, t8264: F, t884: F, t903: F, t9321: F) -> F {
    let t43207 = F::new(0.39726959900411316772e-4) * t39977;
    let t43211 = F::new(0.3193131120497015617e0) * t39997;
    let t43231 = -F::new(0.1702583995731913576e-4) * t39975 - t43207 + F::new(0.5107751987195740728e-4) * t39979 + F::new(0.5107751987195740728e-4) * t39985 + F::new(0.17961362552795712846e0) * t39994 + t43211 + F::new(0.212822999466489197e-4) * t40002 - F::new(0.5107751987195740728e-4) * t40007 + F::new(0.23948483403727617128e0) * t739 * t8264 * t5144 - F::new(0.23948483403727617128e0) * t884 * t8264 * t5267 + F::new(0.17961362552795712846e0) * t903 * t699 * t5199 + F::new(0.23948483403727617128e0) * t2604 * t9321 - F::new(0.32729593985094410076e0) * t40012 - F::new(0.81823984962736025192e-1) * t40015 + F::new(0.16364796992547205038e0) * t40018 + F::new(0.11974241701863808564e0) * t40021 - F::new(0.17961362552795712846e0) * t40024 - F::new(0.11974241701863808564e0) * t40027;
    t43231
}
