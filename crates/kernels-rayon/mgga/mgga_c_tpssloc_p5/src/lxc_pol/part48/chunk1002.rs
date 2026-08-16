//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1002/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1002(t115322: f64, t115364: f64, t115513: f64, t115532: f64, t115570: f64, t115590: f64, t115622: f64, t115660: f64, t1390: f64, t1983: f64, t533: f64, t2075: f64, t22479: f64, t652: f64) -> (f64, f64) {
    let t115666 = t1983 * t533 * (t115322 + t115364 + t115513 + t115532 + t115570 + t115590 + t115622 + t115660) * t1390;
    let t115669 = 2.0_f64 * t652 * t2075 * t22479;
    (t115666, t115669)
}
