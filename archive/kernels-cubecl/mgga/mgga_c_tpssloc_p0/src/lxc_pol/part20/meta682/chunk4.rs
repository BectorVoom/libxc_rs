//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2578/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2578<F: Float>(t11570: F, t12648: F, t10913: F, t14730: F, t1409: F, t3450: F, t3469: F, t14725: F, t15288: F, t15338: F, t3447: F, t1090: F, t11526: F, t11569: F, t11575: F, t11593: F, t15293: F, t15390: F, t15395: F, t24705: F, t3449: F, t44415: F, t44419: F, t44445: F, t44478: F, t44481: F, t44487: F, t4889: F, t4900: F, t4919: F, t50959: F) -> (F, F, F) {
    let t52161 = t11570 * t12648;
    let t52165 = t14730 * t10913;
    let t52170 = t3450 * t1409 * t3469;
    let t52183 = t14725 * t10913;
    let t52191 = t3447 * t15338 * t15288;
    let t52197 = F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4900 * t50959 + F::cast_from(0.74074074074074074073e-3_f64) * t4889 * t11526 - F::cast_from(0.27777777777777777777e-3_f64) * t44445 - F::cast_from(0.9259259259259259259e-3_f64) * t44478 - F::cast_from(0.27777777777777777777e-3_f64) * t44481 - t44487 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t11569 * t52161 + F::cast_from(0.66666666666666666663e-2_f64) * t3447 * t4900 * t52165 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t3449 * t52170 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t11593 * t15293 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t11575 * t15293 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t15390 * t44419 - F::cast_from(0.25925925925925925925e-2_f64) * t3447 * t15395 * t52183 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t4919 * t44415 + F::cast_from(0.55555555555555555554e-3_f64) * t52191 + F::cast_from(0.83333333333333333331e-3_f64) * t3447 * t4919 * t24705 * t1090;
    (t52165, t52183, t52197)
}
