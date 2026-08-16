//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 357/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk357(t2140: f64, t334: f64, t688: f64, t125: f64, t137: f64, t86: f64, t165: f64, t113: f64, t153: f64, t160: f64, t62: f64) -> (f64, f64, f64, f64, f64) {
    let t2141 = t688 * t334 * t2140;
    let t2144 = t86 * t125 * t137;
    let t2146 = -0.69505208333333333333e-3_f64 * t2141 + 0.99491666666666666664e-2_f64 * t2144;
    let t2147 = t2146 * t165;
    let t2148 = t153 * t113;
    let t2150 = t62 * t160;
    (t2144, t2146, t2147, t2148, t2150)
}
