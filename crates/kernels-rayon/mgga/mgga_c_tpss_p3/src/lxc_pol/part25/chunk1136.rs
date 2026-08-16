//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1136/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1136(t15626: f64, t2911: f64, t5081: f64, t9495: f64, t1042: f64, t9493: f64, t4192: f64, t4198: f64, t4181: f64, t4197: f64, t1089: f64, t3009: f64, t5191: f64) -> (f64, f64, f64, f64, f64) {
    let t15628 = 0.32163958997385070134e2_f64 * t2911 * t15626;
    let t15629 = t5081 * t9495;
    let t15630 = t15629 * t1042;
    let t15632 = 0.51726012919273400301e3_f64 * t9493 * t15630;
    let t15634 = 0.23392894490538584828e1_f64 * t4192 * t4198;
    let t15635 = t4197 * t4181;
    let t15637 = 0.23392894490538584828e1_f64 * t1089 * t15635;
    let t15639 = 0.11696447245269292414e1_f64 * t3009 * t5191;
    (t15628, t15632, t15634, t15637, t15639)
}
