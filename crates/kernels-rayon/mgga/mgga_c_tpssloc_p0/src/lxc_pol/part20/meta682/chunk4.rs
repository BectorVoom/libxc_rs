//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2578/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2578(t11570: f64, t12648: f64, t10913: f64, t14730: f64, t1409: f64, t3450: f64, t3469: f64, t14725: f64, t15288: f64, t15338: f64, t3447: f64, t1090: f64, t11526: f64, t11569: f64, t11575: f64, t11593: f64, t15293: f64, t15390: f64, t15395: f64, t24705: f64, t3449: f64, t44415: f64, t44419: f64, t44445: f64, t44478: f64, t44481: f64, t44487: f64, t4889: f64, t4900: f64, t4919: f64, t50959: f64) -> (f64, f64, f64) {
    let t52161 = t11570 * t12648;
    let t52165 = t14730 * t10913;
    let t52170 = t3450 * t1409 * t3469;
    let t52183 = t14725 * t10913;
    let t52191 = t3447 * t15338 * t15288;
    let t52197 = 0.11111111111111111111e-2_f64 * t3447 * t4900 * t50959 + 0.74074074074074074073e-3_f64 * t4889 * t11526 - 0.27777777777777777777e-3_f64 * t44445 - 0.9259259259259259259e-3_f64 * t44478 - 0.27777777777777777777e-3_f64 * t44481 - t44487 - 0.11111111111111111111e-2_f64 * t3447 * t11569 * t52161 + 0.66666666666666666663e-2_f64 * t3447 * t4900 * t52165 + 0.83333333333333333331e-3_f64 * t3447 * t3449 * t52170 + 0.16666666666666666666e-2_f64 * t3447 * t11593 * t15293 + 0.16666666666666666666e-2_f64 * t3447 * t11575 * t15293 - 0.11111111111111111111e-2_f64 * t3447 * t15390 * t44419 - 0.25925925925925925925e-2_f64 * t3447 * t15395 * t52183 + 0.16666666666666666666e-2_f64 * t3447 * t4919 * t44415 + 0.55555555555555555554e-3_f64 * t52191 + 0.83333333333333333331e-3_f64 * t3447 * t4919 * t24705 * t1090;
    (t52165, t52183, t52197)
}
