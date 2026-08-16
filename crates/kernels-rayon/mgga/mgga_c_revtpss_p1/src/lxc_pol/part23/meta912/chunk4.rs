//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2936/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2936(t41610: f64, t63276: f64, t63278: f64, t77507: f64, t77509: f64, t77712: f64, t77715: f64, t77718: f64, t77721: f64, t77724: f64, t77727: f64, t77730: f64) -> f64 {
    let t78023 = -0.39862222222222222223e0_f64 * t77507 + 0.59793333333333333333e0_f64 * t77509 - 0.59793333333333333334e0_f64 * t63276 + 0.19931111111111111111e0_f64 * t63278 + t41610 + 0.16431333333333333333e0_f64 * t77712 - 0.27385555555555555556e-1_f64 * t77715 + 0.43816888888888888889e0_f64 * t77718 - 0.10954222222222222222e0_f64 * t77721 - 0.85199506172839506175e-1_f64 * t77724 + 0.49294e0_f64 * t77727 - 0.10954222222222222222e0_f64 * t77730;
    t78023
}
