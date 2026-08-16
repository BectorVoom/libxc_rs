//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1314/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314(t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t39738: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t215: f64, t2581: f64, t2585: f64, t268: f64) -> (f64, f64) {
    let t39751 = t39520 - t39528 + t39531 + t39534 + t39537 - t39540 + t39738 + t39741 + t39744 + t39747 + t39750;
    let t39756 = 0.22911460125803964958e1_f64 * t268 * t215 * t2581 * t2585;
    (t39751, t39756)
}
