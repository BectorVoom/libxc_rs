//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 449/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk449(t213: f64, t218: f64, t847: f64, t1658: f64, t90: f64, t1654: f64, t215: f64, t851: f64, t1666: f64, t220: f64, t43: f64, t448: f64, t894: f64, t1061: f64, t119: f64, t481: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2237 = t847 * t847;
    let t2241 = 2.0_f64 * t90 + 2.0_f64 * t1658;
    let t2245 = piecewise3(t214, 0.0_f64, 4.0_f64 / 9.0_f64 * t1654 * t2237 + 4.0_f64 / 3.0_f64 * t215 * t2241);
    let t2246 = t851 * t851;
    let t2249 = -t2241;
    let t2253 = piecewise3(t219, 0.0_f64, 4.0_f64 / 9.0_f64 * t1666 * t2246 + 4.0_f64 / 3.0_f64 * t220 * t2249);
    let t2255 = (t2245 + t2253) * t43;
    let t2264 = t894 * t448;
    let t2268 = t481 * t1061 * t119;
    (t2255, t2264, t2268)
}
