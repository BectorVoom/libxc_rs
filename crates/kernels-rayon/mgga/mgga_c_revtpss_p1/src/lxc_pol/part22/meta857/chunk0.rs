//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3003/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3003(t14701: f64, t40731: f64, t14468: f64, t221: f64, t2674: f64, t2675: f64, t14662: f64, t231: f64, t243: f64, t2661: f64, t2662: f64, t14648: f64, t14832: f64, t2430: f64) -> (f64, f64, f64, f64) {
    let t50298 = t40731 * t14701;
    let t50303 = t2674 * t2675 * t221 * t14468;
    let t50308 = t2661 * t2662 * t243 * t14662 * t231;
    let t50312 = t2661 * t14832 * t14648 * t2430;
    (t50298, t50303, t50308, t50312)
}
