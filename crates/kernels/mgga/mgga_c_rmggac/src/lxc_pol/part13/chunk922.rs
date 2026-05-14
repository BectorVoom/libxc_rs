//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 922/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk922<F: Float>(t39544: F, t1364: F, t1550: F, t1624: F, t1627: F, t1632: F, t1635: F, t2228: F, t39538: F, t39541: F, t39547: F, t39549: F, t39555: F, t39559: F, t39561: F, t39563: F, t39566: F, t39568: F, t39571: F, t739: F, t8264: F, t8377: F, t903: F) -> (F,) {
    let t43008 = 0.47896966807455234256e0 * t39544;
    let t43033 = -0.71845450211182851384e0 * t39538 + 0.17961362552795712846e0 * t39541 + t43008 - 0.8980681276397856423e-1 * t39547 + 0.35922725105591425692e0 * t39549 - 0.20496175532535769482e-3 * t39555 + 0.5107751987195740728e-4 * t39559 + 0.1702583995731913576e-4 * t39561 + 0.5454932330849068346e-1 * t39563 + 0.40911992481368012596e-1 * t39566 - 0.23948483403727617128e0 * t1550 * t2228 * t1624 + 0.35922725105591425692e0 * t903 * t2228 * t1627 + 0.23948483403727617128e0 * t739 * t8264 * t8377 - 0.5987120850931904282e-1 * t39568 + 0.5454932330849068346e-1 * t39571 + 0.35922725105591425692e0 * t903 * t2228 * t1632 - 0.47896966807455234256e0 * t1364 * t2228 * t1635;
    (t43033,)
}
