//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1056/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1056(t39544: f64, t1364: f64, t1550: f64, t1624: f64, t1627: f64, t1632: f64, t1635: f64, t2228: f64, t39538: f64, t39541: f64, t39547: f64, t39549: f64, t39555: f64, t39559: f64, t39561: f64, t39563: f64, t39566: f64, t39568: f64, t39571: f64, t739: f64, t8264: f64, t8377: f64, t903: f64) -> f64 {
    let t43008 = 0.47896966807455234256e0_f64 * t39544;
    let t43033 = -0.71845450211182851384e0_f64 * t39538 + 0.17961362552795712846e0_f64 * t39541 + t43008 - 0.8980681276397856423e-1_f64 * t39547 + 0.35922725105591425692e0_f64 * t39549 - 0.20496175532535769482e-3_f64 * t39555 + 0.5107751987195740728e-4_f64 * t39559 + 0.1702583995731913576e-4_f64 * t39561 + 0.5454932330849068346e-1_f64 * t39563 + 0.40911992481368012596e-1_f64 * t39566 - 0.23948483403727617128e0_f64 * t1550 * t2228 * t1624 + 0.35922725105591425692e0_f64 * t903 * t2228 * t1627 + 0.23948483403727617128e0_f64 * t739 * t8264 * t8377 - 0.5987120850931904282e-1_f64 * t39568 + 0.5454932330849068346e-1_f64 * t39571 + 0.35922725105591425692e0_f64 * t903 * t2228 * t1632 - 0.47896966807455234256e0_f64 * t1364 * t2228 * t1635;
    t43033
}
