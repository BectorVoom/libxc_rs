//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1494/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1494(t22335: f64, t2470: f64, t4101: f64, t10073: f64, t22361: f64, t10069: f64, t22373: f64, t10139: f64, t136: f64, t2457: f64, t6874: f64, t6844: f64) -> (f64, f64, f64, f64, f64) {
    let t75092 = t4101 * t22335 * t2470;
    let t75113 = t10073 * t22361;
    let t75119 = t10069 * t22373;
    let t75123 = t10139 * t6874 * t136 * t2457;
    let t75128 = t10139 * t6844 * t136 * t2457;
    (t75092, t75113, t75119, t75123, t75128)
}
