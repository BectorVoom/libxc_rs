//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2086/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2086(t10301: f64, t1470: f64, t2247: f64, t4181: f64, t4187: f64, t28019: f64, t531: f64, t1513: f64, t94975: f64, t28036: f64, t94978: f64, t25823: f64, t4287: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    let t101243 = t2247 * t4187;
    let t101417 = t531 * t28019;
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    let t101454 = 4.0_f64 / 3.0_f64 * t101453;
    let t101455 = t25823 * t4287;
    (t101237, t101240, t101243, t101417, t101451, t101454, t101455)
}
