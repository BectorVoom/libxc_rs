//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1291/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1291(t12825: f64, t7978: f64, t8221: f64, t27591: f64, t28727: f64, t28714: f64, t2104: f64, t27614: f64, t4468: f64, t6176: f64, t27556: f64, t27595: f64, t27617: f64, t27638: f64, t28738: f64, t7968: f64, t94914: f64, t94916: f64, t99074: f64, t99079: f64) -> (f64, f64) {
    let t99152 = t7978 * t12825 * t8221;
    let t99154 = t28727 * t27591;
    let t99157 = 0.7722800925925925926e-4_f64 * t28714 * t27591;
    let t99166 = t6176 * t27614 * t2104 * t4468;
    let t99169 = 0.18534722222222222222e-2_f64 * t28727 * t27638 + 0.11584201388888888889e-3_f64 * t94914 + 0.30918233506944444444e-4_f64 * t94916 - 0.69505208333333333334e-3_f64 * t28714 * t27617 + 0.25742669753086419753e-4_f64 * t99152 + 0.20594135802469135802e-3_f64 * t99154 - t99157 - 0.13913205078125e-3_f64 * t7968 * t99074 + 0.557015165302734375e-4_f64 * t27595 * t99079 - 0.92754700520833333334e-4_f64 * t27556 * t28738 - 0.46377350260416666667e-4_f64 * t7968 * t99166;
    (t99166, t99169)
}
