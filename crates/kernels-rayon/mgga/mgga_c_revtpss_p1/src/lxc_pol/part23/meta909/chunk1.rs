//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2920/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920(t42518: f64, t52011: f64, t77513: f64, t41307: f64, t63276: f64, t63278: f64, t77507: f64, t77509: f64, t77712: f64, t77715: f64, t77718: f64, t77721: f64, t77724: f64, t77727: f64) -> (f64, f64) {
    let t77730 = t52011 * t42518 * t77513;
    let t77732 = -0.40256666666666666667e0_f64 * t77507 + 0.60385e0_f64 * t77509 - 0.60385000000000000002e0_f64 * t63276 + 0.20128333333333333334e0_f64 * t63278 + t41307 + 0.16557e0_f64 * t77712 - 0.27595e-1_f64 * t77715 + 0.44152e0_f64 * t77718 - 0.11038e0_f64 * t77721 - 0.8585111111111111111e-1_f64 * t77724 + 0.49671e0_f64 * t77727 - 0.11038e0_f64 * t77730;
    (t77730, t77732)
}
