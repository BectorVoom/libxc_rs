//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 939/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk939<F: Float>(t34976: F, t40145: F, t4550: F, t495: F, t8440: F, t35039: F, t39851: F, t498: F, t16504: F, t321: F, t333: F, t3369: F) -> (F, F, F, F) {
    let t40149 = t40145 * t34976 * t8440 * t4550 * t495;
    let t40154 = t39851 * t35039 * t8440 * t4550 * t498;
    let t40159 = t39851 * t16504 * t8440 * t4550 * t321;
    let t40164 = t39851 * t3369 * t8440 * t4550 * t333;
    (t40149, t40154, t40159, t40164)
}
