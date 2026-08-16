//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3188/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188(t1261: f64, t12879: f64, t247: f64, t5056: f64, t12963: f64, t5323: f64, t225: f64, t56587: f64, t17795: f64, t3172: f64, t3711: f64, t17729: f64, t17759: f64, t44425: f64) -> (f64, f64, f64, f64, f64) {
    let t59233 = t1261 * t247 * t12879 * t5056;
    let t59239 = t5323 * t12963;
    let t59241 = t56587 * t225;
    let t59269 = t3711 * t3172 * t17795;
    let t59320 = t17729 * t44425 * t17759;
    (t59233, t59239, t59241, t59269, t59320)
}
