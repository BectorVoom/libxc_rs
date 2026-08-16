//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 996/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk996(t1166: f64, t4869: f64, t1703: f64, t3411: f64, t1694: f64, t3375: f64, t1157: f64, t1164: f64, t1147: f64, t1156: f64, t4857: f64, t3400: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4871 = 0.5848223622634646207e0_f64 * t4869 * t1166;
    let t4873 = 0.5848223622634646207e0_f64 * t3411 * t1703;
    let t4874 = t3375 * t1694;
    let t4875 = t4874 * t1157;
    let t4877 = 0.11696447245269292414e1_f64 * t1164 * t4875;
    let t4879 = t1147 * t4857 * t1156;
    let t4881 = 0.5848223622634646207e0_f64 * t1164 * t4879;
    let t4882 = t3400 * t1694;
    (t4871, t4873, t4874, t4875, t4877, t4879, t4881, t4882)
}
