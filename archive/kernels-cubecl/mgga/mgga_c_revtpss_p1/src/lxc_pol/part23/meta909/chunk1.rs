//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2920/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2920<F: Float>(t42518: F, t52011: F, t77513: F, t41307: F, t63276: F, t63278: F, t77507: F, t77509: F, t77712: F, t77715: F, t77718: F, t77721: F, t77724: F, t77727: F) -> (F, F) {
    let t77730 = t52011 * t42518 * t77513;
    let t77732 = -F::cast_from(0.40256666666666666667e0_f64) * t77507 + F::cast_from(0.60385e0_f64) * t77509 - F::cast_from(0.60385000000000000002e0_f64) * t63276 + F::cast_from(0.20128333333333333334e0_f64) * t63278 + t41307 + F::cast_from(0.16557e0_f64) * t77712 - F::cast_from(0.27595e-1_f64) * t77715 + F::cast_from(0.44152e0_f64) * t77718 - F::cast_from(0.11038e0_f64) * t77721 - F::cast_from(0.8585111111111111111e-1_f64) * t77724 + F::cast_from(0.49671e0_f64) * t77727 - F::cast_from(0.11038e0_f64) * t77730;
    (t77730, t77732)
}
