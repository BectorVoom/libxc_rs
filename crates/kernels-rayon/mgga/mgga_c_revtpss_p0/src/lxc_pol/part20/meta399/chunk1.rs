//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1480/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1480(t3059: f64, t3075: f64, t41306: f64, t41308: f64, t41312: f64, t41316: f64, t41320: f64, t41323: f64, t41327: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64) -> (f64, f64) {
    let t42001 = t3059 * t3075;
    let t42013 = 0.86419753086419753087e-1_f64 * t41306;
    let t42018 = 0.66666666666666666668e-1_f64 * t41308 + 0.2e0_f64 * t41312 - 0.3e0_f64 * t41316 + 0.50000000000000000001e-1_f64 * t41320 + 0.19999999999999999999e0_f64 * t41323 - 0.16666666666666666666e-1_f64 * t41327 + t42013 - 0.22222222222222222222e-1_f64 * t41330 - 0.14814814814814814815e-1_f64 * t41332 + 0.11111111111111111111e-1_f64 * t41334 + 0.12345679012345679012e-1_f64 * t41336;
    (t42001, t42018)
}
