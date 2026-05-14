//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 830/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk830<F: Float>(t1971: F, t209: F, t236: F, t40064: F, t7453: F, t1175: F, t1475: F, t36336: F, t36343: F, t9147: F, t1620: F, t1986: F, t7720: F, t7487: F, t8343: F, t8358: F) -> (F, F, F, F, F, F) {
    let t40068 = t7453 * t1971 * t236 * t40064 * t209;
    let t40073 = t36336 * t1971 * t236 * t1475 * t1175;
    let t40075 = t36343 * t9147;
    let t40076 = 0.24829349937757072982e-4 * t40075;
    let t40081 = t1986 * t1620;
    let t40082 = t7720 * t40081;
    let t40084 = t7487 * t8343;
    let t40085 = 0.19211284388664477842e-2 * t40084;
    let t40086 = t7487 * t8358;
    (t40068, t40073, t40076, t40082, t40085, t40086)
}
