//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 735/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk735<F: Float>(t2001: F, t2281: F, t305: F, t321: F, t2286: F, t34881: F, t16156: F, t9051: F, t36343: F, t9147: F, t1620: F, t1986: F, t7487: F, t8343: F, t8358: F, t8362: F) -> (F, F, F, F, F, F, F, F) {
    let t40031 = t2001 * t305 * t2281 * t321;
    let t40045 = t34881 * t2286;
    let t40062 = t16156 * t9051;
    let t40063 = 0.19863479950205658386e-4 * t40062;
    let t40075 = t36343 * t9147;
    let t40076 = 0.24829349937757072982e-4 * t40075;
    let t40081 = t1986 * t1620;
    let t40084 = t7487 * t8343;
    let t40085 = 0.19211284388664477842e-2 * t40084;
    let t40086 = t7487 * t8358;
    let t40087 = 0.19211284388664477842e-2 * t40086;
    let t40088 = t7487 * t8362;
    (t40031, t40045, t40063, t40076, t40081, t40085, t40087, t40088)
}
