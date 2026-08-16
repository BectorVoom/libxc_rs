//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 295/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk295(t12: f64, t24: f64, t1009: f64, t83: f64, t1008: f64, t124: f64, t207: f64, t972: f64, t1003: f64, t333: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t1010 = t83 * t1009;
    let t1012 = 0.19751673498613801407e-1_f64 * t1008 * t124;
    let t1015 = piecewise3(t84, 0.0_f64, 2.0_f64 / 3.0_f64 * t207 * t972);
    let t1018 = piecewise3(t90, 0.0_f64, 2.0_f64 / 3.0_f64 * t333 * t1003);
    let t1020 = t1015 / 2.0_f64 + t1018 / 2.0_f64;
    (t1010, t1012, t1020)
}
