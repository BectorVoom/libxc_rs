//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3414/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3414<F: Float>(t19167: F, t2982: F, t63618: F, t63620: F, t63622: F, t63625: F, t63628: F, t63633: F, t63636: F, t63638: F, t63641: F, t63644: F, t63647: F, t63649: F, t63653: F, t63660: F, t63668: F, t63670: F, t63673: F, t63816: F, t965: F, t973: F) -> F {
    let t64152 = t63618 - t63620 - t63622 - t63625 - t63628 + t63633 + t63636 + t63638 + t63641 + t63644 + t63647 - t63649 - t63653 - t63660 + F::cast_from(0.11696447245269292414e1_f64) * t2982 * t19167 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t63816 * t973 + t63668 + t63670 + t63673;
    t64152
}
