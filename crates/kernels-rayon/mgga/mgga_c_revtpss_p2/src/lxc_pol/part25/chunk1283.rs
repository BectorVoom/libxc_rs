//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1283/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1283(t11240: f64, t11244: f64, t11627: f64, t25503: f64, t11273: f64, t25508: f64, t25526: f64, t3173: f64, t11263: f64, t7122: f64, t11762: f64, t7111: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93789 = t11240 * t11627 * sigma0 * t11244;
    let t93793 = t11240 * t25503 * t11244;
    let t93796 = t11273 * t25508;
    let t93799 = t25526 * t3173;
    let t93801 = t7122 * t11263;
    let t93813 = t7111 * t11762;
    (t93789, t93793, t93796, t93799, t93801, t93813)
}
