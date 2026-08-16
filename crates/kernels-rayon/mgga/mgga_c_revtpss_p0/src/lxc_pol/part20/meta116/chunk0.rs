//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 672/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk672(t1025: f64, t3215: f64, t3075: f64, t373: f64, t371: f64, t372: f64, t225: f64, t3046: f64) -> (f64, f64, f64, f64) {
    let t3216 = t1025 * t3215;
    let t3218 = t373 * t3075;
    let t3220 = t371 * t372 * t3218;
    let t3223 = t3046 * t225;
    (t3216, t3218, t3220, t3223)
}
