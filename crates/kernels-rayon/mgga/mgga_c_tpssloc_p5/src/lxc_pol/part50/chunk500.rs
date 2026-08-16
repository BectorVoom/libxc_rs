//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 500/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk500(t1012: f64, t3108: f64, t1009: f64, t990: f64, t1011: f64, t1019: f64, t1004: f64, t1040: f64, t1013: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3109 = t1012 * t3108;
    let t3112 = t990 * t1009;
    let t3113 = t3112 * t1011;
    let t3114 = t3113 * t1019;
    let t3117 = t1004 * t1040;
    let t3127 = 1.0_f64 / t1013 / t361;
    (t3109, t3112, t3113, t3114, t3117, t3127)
}
