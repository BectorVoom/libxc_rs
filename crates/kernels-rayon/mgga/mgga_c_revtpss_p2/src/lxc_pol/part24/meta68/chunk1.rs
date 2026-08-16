//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 425/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk425(t1163: f64, t1166: f64, t1717: f64, t1724: f64, t1727: f64, t1730: f64) -> f64 {
    let t1744 = 0.3529725e1_f64 * t1724 - t1163 + 0.516475e0_f64 * t1717 + 0.6311625e0_f64 * t1727 - t1166 + 0.104195e0_f64 * t1730;
    t1744
}
