//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 587/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk587(t1023: f64, t248: f64, t3101: f64, t1020: f64, t1017: f64, t1030: f64, t1015: f64, t1012: f64, t1009: f64, t990: f64, t1011: f64, t1019: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3107 = t1030 * t1017;
    let t3108 = t1015 * t3107;
    let t3109 = t1012 * t3108;
    let t3112 = t990 * t1009;
    let t3113 = t3112 * t1011;
    let t3114 = t3113 * t1019;
    (t3103, t3104, t3108, t3109, t3112, t3114)
}
