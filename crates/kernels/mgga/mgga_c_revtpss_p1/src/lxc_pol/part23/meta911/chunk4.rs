//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2930/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2930<F: Float>(t41690: F, t63276: F, t63278: F, t77507: F, t77509: F, t77712: F, t77715: F, t77718: F, t77721: F, t77724: F, t77727: F, t77730: F) -> F {
    let t77911 = -F::cast_from(0.68863333333333333333e0_f64) * t77507 + F::new(0.103295e1) * t77509 - F::new(0.103295e1) * t63276 + F::cast_from(0.34431666666666666666e0_f64) * t63278 + t41690 + F::new(0.20839e0) * t77712 - F::cast_from(0.34731666666666666667e-1_f64) * t77715 + F::cast_from(0.55570666666666666666e0_f64) * t77718 - F::cast_from(0.13892666666666666667e0_f64) * t77721 - F::cast_from(0.10805407407407407407e0_f64) * t77724 + F::new(0.62517e0) * t77727 - F::cast_from(0.13892666666666666667e0_f64) * t77730;
    t77911
}
