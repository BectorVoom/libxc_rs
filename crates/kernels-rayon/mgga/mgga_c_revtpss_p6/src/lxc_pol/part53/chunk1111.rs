//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1111/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1111(t121035: f64, t25875: f64, t122: f64, t72: f64, t8578: f64, t3916: f64, t121072: f64, t2453: f64, t32217: f64, t25304: f64, t32237: f64, t136: f64, t2457: f64, t8585: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t121131 = t25875 * t121035;
    let t121133 = t8578 * t72 * t122;
    let t121134 = t121133 * t3916;
    let t121135 = t121131 * t121134;
    let t121139 = 0.3427046870806409921e-2_f64 * t2453 * t32217 * t121072;
    let t121140 = t25304 * t32237;
    let t121142 = t8585 * t136 * t2457;
    (t121131, t121133, t121134, t121135, t121139, t121140, t121142)
}
