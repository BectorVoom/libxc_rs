//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1009/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1009(t10550: f64, t10571: f64, t10590: f64, t10615: f64, t225: f64, t2475: f64, t73: f64, t2394: f64, t775: f64) -> (f64, f64, f64) {
    let t10618 = (t10550 + t10571 + t10590 + t10615) * t225;
    let t10626 = t73 * t2475;
    let t10627 = t2394 * t775;
    (t10618, t10626, t10627)
}
