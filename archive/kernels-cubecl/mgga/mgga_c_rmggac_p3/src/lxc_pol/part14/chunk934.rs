//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 934/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk934<F: Float>(t40075: F, t1620: F, t1986: F, t7720: F, t7487: F, t8343: F, t8358: F, t8362: F, t2001: F, t2281: F, t326: F, t333: F) -> (F, F, F, F, F, F) {
    let t40076 = F::cast_from(0.24829349937757072982e-4_f64) * t40075;
    let t40081 = t1986 * t1620;
    let t40082 = t7720 * t40081;
    let t40084 = t7487 * t8343;
    let t40085 = F::cast_from(0.19211284388664477842e-2_f64) * t40084;
    let t40086 = t7487 * t8358;
    let t40087 = F::cast_from(0.19211284388664477842e-2_f64) * t40086;
    let t40088 = t7487 * t8362;
    let t40089 = F::cast_from(0.19211284388664477842e-2_f64) * t40088;
    let t40092 = t2001 * t326 * t2281 * t333;
    (t40076, t40082, t40085, t40087, t40089, t40092)
}
