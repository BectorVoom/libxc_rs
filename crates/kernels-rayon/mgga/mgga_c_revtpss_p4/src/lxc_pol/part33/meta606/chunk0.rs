//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2030/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2030(t26866: f64, t3746: f64, t12904: f64, t7618: f64, t3666: f64, t7623: f64, t12808: f64, t29096: f64, t3655: f64, t7610: f64, t12898: f64, t2139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97232 = t3746 * t26866;
    let t97247 = t7618 * t12904;
    let t97250 = t3666 * t7623;
    let t97261 = t12808 * t29096;
    let t97267 = t7610 * t3655;
    let t97272 = 0.1270341277572436651e-3_f64 * t2139 * t12898;
    (t97232, t97247, t97250, t97261, t97267, t97272)
}
