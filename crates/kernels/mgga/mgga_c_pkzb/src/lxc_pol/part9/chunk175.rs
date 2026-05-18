//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 175/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk175<F: Float>(t110: F, t123: F, t466: F, t469: F, t49: F, t494: F, t520: F, t527: F, t535: F, t542: F) -> F {
    let t545 = F::new(0.53237641966666666666e-3) * t49 * t466 * t110 + F::new(1.0) * t520 * t527 - t469 - t494 + F::new(0.18311447306006545054e-3) * t49 * t466 * t123 + F::new(0.5848223622634646207e0) * t535 * t542;
    t545
}
