//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3035/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035(t11044: f64, t14983: f64, t14485: f64, t15014: f64, t9303: f64, t10510: f64, t14987: f64, t14991: f64, t41066: f64, t10982: f64, t1568: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51231 = t11044 * t14983;
    let t51233 = t11044 * t14485;
    let t51237 = t9303 * t15014;
    let t51239 = t14987 * t10510;
    let t51241 = t41066 * t14991;
    let t51246 = t9646 * t1568 * t10982;
    (t51231, t51233, t51237, t51239, t51241, t51246)
}
