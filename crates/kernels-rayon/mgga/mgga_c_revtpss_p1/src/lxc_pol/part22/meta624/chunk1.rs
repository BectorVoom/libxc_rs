//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2538/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2538(t19920: f64, t3127: f64, t1011: f64, t11881: f64, t15986: f64, t15990: f64, t15996: f64, t16037: f64, t19908: f64, t19913: f64, t19917: f64, t3241: f64, t6289: f64, t6293: f64) -> f64 {
    let t19921 = t3127 * t19920;
    let t19923 = -t3241 * t6289 / 108.0_f64 + t19908 / 864.0_f64 - t3241 * t6293 / 81.0_f64 + t19913 / 648.0_f64 - t11881 / 1296.0_f64 + t15986 - t15990 + t15996 - t16037 + t1011 * t19917 / 288.0_f64 - 0.19055119163586549765e-3_f64 * t19921;
    t19923
}
