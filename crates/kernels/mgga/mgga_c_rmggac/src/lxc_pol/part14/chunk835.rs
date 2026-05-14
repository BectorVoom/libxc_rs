//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 835/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk835<F: Float>(t35039: F, t39851: F, t4550: F, t498: F, t8440: F, t16504: F, t321: F, t333: F, t3369: F, t109: F, t24890: F, t490: F, t1001: F, t236: F, t3351: F, t618: F) -> (F, F, F, F) {
    let t40154 = t39851 * t35039 * t8440 * t4550 * t498;
    let t40159 = t39851 * t16504 * t8440 * t4550 * t321;
    let t40164 = t39851 * t3369 * t8440 * t4550 * t333;
    let t40167 = t24890 * t109;
    let t40168 = t490 * t40167;
    let t40172 = t3351 * t40168 * t236 * t618 * t1001;
    (t40154, t40159, t40164, t40172)
}
