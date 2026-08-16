//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1969/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1969(t11569: f64, t15382: f64, t1174: f64, t15332: f64, t15335: f64, t15341: f64, t15360: f64, t15364: f64, t15366: f64, t15374: f64, t15376: f64, t15379: f64, t3447: f64, t3452: f64, t3472: f64, t3478: f64, t4889: f64) -> (f64, f64) {
    let t15383 = t11569 * t15382;
    let t15386 = -0.55555555555555555554e-3_f64 * t3447 * t15332 - 0.16666666666666666666e-2_f64 * t3447 * t15335 + t15341 - 0.83333333333333333332e-3_f64 * t1174 * t15360 + 0.18518518518518518518e-3_f64 * t15364 + 0.14814814814814814815e-2_f64 * t15366 + 0.22222222222222222222e-2_f64 * t4889 * t3472 + 0.22222222222222222222e-2_f64 * t4889 * t3478 - t15374 - 0.14814814814814814815e-2_f64 * t15376 * t3452 + 0.27777777777777777777e-3_f64 * t3447 * t15379 - 0.74074074074074074072e-3_f64 * t3447 * t15383;
    (t15383, t15386)
}
