//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3123/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3123(t11988: f64, t4834: f64, t15731: f64, t3124: f64, t11933: f64, t15794: f64, t3115: f64, t42793: f64, t4911: f64, t11951: f64, t4858: f64, t11922: f64, t15906: f64, t15909: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55272 = t4834 * t11988;
    let t55279 = t3124 * t15731;
    let t55290 = t11933 * t15794;
    let t55293 = t3115 * t42793 * t4911;
    let t55320 = t4858 * t11951;
    let t55325 = t15906 * t11922 * t15909;
    (t55272, t55279, t55290, t55293, t55320, t55325)
}
