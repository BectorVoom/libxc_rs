//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3414/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3414(t19167: f64, t2982: f64, t63618: f64, t63620: f64, t63622: f64, t63625: f64, t63628: f64, t63633: f64, t63636: f64, t63638: f64, t63641: f64, t63644: f64, t63647: f64, t63649: f64, t63653: f64, t63660: f64, t63668: f64, t63670: f64, t63673: f64, t63816: f64, t965: f64, t973: f64) -> f64 {
    let t64152 = t63618 - t63620 - t63622 - t63625 - t63628 + t63633 + t63636 + t63638 + t63641 + t63644 + t63647 - t63649 - t63653 - t63660 + 0.11696447245269292414e1_f64 * t2982 * t19167 + 0.5848223622634646207e0_f64 * t965 * t63816 * t973 + t63668 + t63670 + t63673;
    t64152
}
