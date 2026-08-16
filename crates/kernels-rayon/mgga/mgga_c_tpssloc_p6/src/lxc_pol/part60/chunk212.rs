//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 212/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk212(t1010: f64, t1057: f64, t357: f64, t360: f64, t390: f64, t268: f64, t405: f64, t878: f64) -> (f64, f64, f64, f64) {
    let t1058 = t1010 * t1057;
    let t1060 = t357 * t360;
    let t1070 = 1.0_f64 / t390;
    let t1086 = t268 * t878 * t405;
    (t1058, t1060, t1070, t1086)
}
