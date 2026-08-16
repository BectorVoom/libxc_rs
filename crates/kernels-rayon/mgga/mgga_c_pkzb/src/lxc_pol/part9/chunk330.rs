//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 330/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk330(t1072: f64, t1086: f64, t1088: f64, t1096: f64, t1101: f64, t1108: f64, t237: f64, t248: f64, t695: f64, t714: f64, t1107: f64, t713: f64, t722: f64) -> (f64, f64, f64) {
    let t1112 = t237 * (-0.310907e-1_f64 * t1088 * t248 + 1.0_f64 * t695 * t1096 + t1072 - t1086 - 0.19751673498613801407e-1_f64 * t1101 + 0.5848223622634646207e0_f64 * t714 * t1108);
    let t1114 = 0.19751673498613801407e-1_f64 * t237 * t1101;
    let t1116 = t713 * t1107 * t722;
    (t1112, t1114, t1116)
}
