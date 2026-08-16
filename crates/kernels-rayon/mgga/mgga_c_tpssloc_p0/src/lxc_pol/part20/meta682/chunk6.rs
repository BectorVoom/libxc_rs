//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2580/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2580(t1090: f64, t11569: f64, t1174: f64, t1184: f64, t15288: f64, t15320: f64, t15357: f64, t15382: f64, t15390: f64, t24698: f64, t3243: f64, t3248: f64, t3252: f64, t3447: f64, t3449: f64, t3469: f64, t44499: f64, t44502: f64, t44529: f64, t460: f64, t4908: f64, t4919: f64, t4928: f64, t4934: f64, t52216: f64, t52220: f64, t52224: f64, t52228: f64, t52236: f64, t52240: f64, t52250: f64, t7319: f64) -> f64 {
    let t52257 = 0.16666666666666666666e-2_f64 * t3447 * t15320 * t15288 + 0.83333333333333333331e-3_f64 * t3447 * t4919 * t24698 * t1090 + 0.83333333333333333331e-3_f64 * t3447 * t4919 * t7319 * t3252 + 0.16666666666666666666e-2_f64 * t3447 * t4919 * t7319 * t3248 - 0.11111111111111111111e-2_f64 * t3447 * t44529 * t15382 + 0.33333333333333333333e-2_f64 * t3447 * t3449 * t52216 + 0.16666666666666666666e-2_f64 * t3447 * t3449 * t52220 + 0.49999999999999999999e-2_f64 * t3447 * t3449 * t52224 - 0.66666666666666666665e-2_f64 * t3447 * t11569 * t52228 - 0.11111111111111111111e-2_f64 * t3447 * t15390 * t7319 * t3243 - 0.49999999999999999998e-2_f64 * t3447 * t4908 * t52236 + 0.14814814814814814815e-2_f64 * t52240 - 0.24999999999999999999e-2_f64 * t1174 * t4934 * t15357 * t1184 * t460 + 0.74074074074074074072e-3_f64 * t44499 - 0.55555555555555555554e-3_f64 * t44502 - 0.16666666666666666666e-2_f64 * t52250 - 0.24999999999999999999e-2_f64 * t1174 * t4934 * t4928 * t3469 * t460;
    t52257
}
