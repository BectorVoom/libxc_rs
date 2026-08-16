//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 340/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk340(t1173: f64, t1187: f64, t1189: f64, t1197: f64, t1202: f64, t1209: f64, t237: f64, t365: f64, t863: f64, t882: f64, t1208: f64, t881: f64, t890: f64) -> (f64, f64, f64) {
    let t1213 = t237 * (-0.310907e-1_f64 * t1189 * t365 + 1.0_f64 * t863 * t1197 + t1173 - t1187 - 0.19751673498613801407e-1_f64 * t1202 + 0.5848223622634646207e0_f64 * t882 * t1209);
    let t1215 = 0.19751673498613801407e-1_f64 * t237 * t1202;
    let t1217 = t881 * t1208 * t890;
    (t1213, t1215, t1217)
}
