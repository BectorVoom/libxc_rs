//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 956/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk956(t11244: f64, t3144: f64, t11240: f64, t1043: f64, t3151: f64, t373: f64, t3153: f64, t73: f64) -> (f64, f64, f64, f64) {
    let t11245 = t3144 * t11244;
    let t11246 = t11240 * t11245;
    let t11247 = t3151 * t1043;
    let t11248 = t373 * t11247;
    let t11249 = t3153 * t73;
    (t11246, t11247, t11248, t11249)
}
