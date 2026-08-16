//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2170/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2170<F: Float>(t11277: F, t3307: F, t11285: F, t3395: F, t43776: F, t43819: F, t3311: F, t409: F, t3314: F, t11399: F, t3403: F, t11352: F, t3351: F) -> (F, F, F, F, F, F, F, F) {
    let t43976 = t3307 * t11277;
    let t43984 = t11285 * t3395;
    let t44027 = F::cast_from(0.13388493827160493828e1_f64) * t43776;
    let t44053 = F::cast_from(0.31003950617283950618e1_f64) * t43819;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = F::cast_from(1.0_f64) / t44076;
    let t44106 = t11399 * t3403;
    let t44131 = t3351 * t11352;
    (t43976, t43984, t44027, t44053, t44075, t44077, t44106, t44131)
}
