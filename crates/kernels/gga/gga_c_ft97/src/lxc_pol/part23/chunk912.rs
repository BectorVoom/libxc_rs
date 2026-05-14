//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 912/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk912<F: Float>(t10085: F, t6848: F, t1091: F, t24747: F, t2599: F, t3746: F, t6074: F, t14196: F, t27757: F, t1456: F, t3821: F, t729: F, t6947: F, t713: F, t1175: F, t6061: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28150 = t10085 * t6848;
    let t28153 = t24747 * t1091;
    let t28154 = t2599 * t28153;
    let t28157 = t6074 * t3746;
    let t28158 = t2599 * t28157;
    let t28163 = t14196 * t27757;
    let t28167 = t729 * t1456 * t3821;
    let t28171 = t729 * t6947 * t713;
    let t28175 = t729 * t1175 * t6061;
    (t28150, t28153, t28154, t28157, t28158, t28163, t28167, t28171, t28175)
}
