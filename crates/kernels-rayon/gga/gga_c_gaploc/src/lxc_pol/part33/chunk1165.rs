//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1165/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1165(t31590: f64, t6508: f64, t1358: f64, t6507: f64, t10269: f64, t3808: f64, t29896: f64, t29898: f64, t29901: f64, t29903: f64, t29908: f64, t29911: f64, t29913: f64, t29915: f64, t471: f64) -> (f64, f64, f64, f64) {
    let t31591 = t6508 * t31590;
    let t31594 = 0.12646669615856066488e-1_f64 * t1358 * t6507 * t31591;
    let t31600 = 0.12646669615856066488e-1_f64 * t3808 * t10269;
    let t31610 = (189.0_f64 / 512.0_f64 * t29896 - 2499.0_f64 / 16384.0_f64 * t29898 + 1239.0_f64 / 524288.0_f64 * t29901 - 441.0_f64 / 0.16777216e8_f64 * t29903 + 147.0_f64 / 0.16777216e8_f64 * t29908 - 413.0_f64 / 524288.0_f64 * t29911 + 833.0_f64 / 16384.0_f64 * t29913 - 63.0_f64 / 512.0_f64 * t29915) * t471;
    (t31591, t31594, t31600, t31610)
}
