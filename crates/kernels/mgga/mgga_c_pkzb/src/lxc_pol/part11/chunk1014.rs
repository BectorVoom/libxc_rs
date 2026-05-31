//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1014/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1014<F: Float>(t11155: F, t6088: F, t7955: F, t9782: F, t352: F, t6127: F, t378: F, t1196: F, t3779: F, t6290: F, t1208: F, t3806: F) -> (F, F, F, F, F, F, F) {
    let t11157 = -t6088 + F::cast_from(0.71233333333333333332e-1_f64) * t7955 - F::cast_from(0.53424999999999999999e-1_f64) * t9782 + F::cast_from(0.53425e-1_f64) * t11155;
    let t11159 = F::cast_from(0.621814e-1_f64) * t11157 * t352;
    let t11163 = -t6127 + F::cast_from(0.37083333333333333334e-1_f64) * t7955 - F::cast_from(0.278125e-1_f64) * t9782 + F::cast_from(0.278125e-1_f64) * t11155;
    let t11164 = t11163 * t378;
    let t11166 = t3779 * t1196;
    let t11167 = t11166 * t6290;
    let t11180 = t3806 * t1208;
    (t11157, t11159, t11163, t11164, t11166, t11167, t11180)
}
