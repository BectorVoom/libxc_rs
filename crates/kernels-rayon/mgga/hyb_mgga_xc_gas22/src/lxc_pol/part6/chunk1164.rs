//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1164/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1164(t136: f64, t215: f64, t8184: f64, t2004: f64, t2011: f64, t19746: f64, t222: f64, t226: f64, t12: f64, t5: f64, t231: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20578 = 5.0_f64 / 108.0_f64 * t136 * t8184 * t215;
    let t20579 = t2004 * t2011;
    let t20624 = t222 * t19746 * t226;
    let t20625 = 0.31310740740740740741e1_f64 * t20624;
    let t20626 = t12 * t5;
    let t20631 = 1.0_f64 / t231 / t20626 / t243 / t226 / 96.0_f64;
    (t20578, t20579, t20624, t20625, t20626, t20631)
}
