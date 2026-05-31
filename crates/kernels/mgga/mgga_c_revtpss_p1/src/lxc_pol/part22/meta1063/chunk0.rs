//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3804/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3804<F: Float>(t1298: F, t5501: F, t18134: F, t5023: F, t68700: F, t68703: F, t68707: F, t68709: F, t68711: F, t68714: F, t68716: F, t68718: F, t68723: F, t68725: F, t68727: F, t68730: F, t68733: F) -> F {
    let t73262 = t5501 * t1298;
    let t73266 = F::cast_from(8.0_f64) * t18134 * t5023 * t73262 - t68700 - t68703 + t68707 - t68709 + t68711 + t68714 - t68716 - t68718 - t68723 + t68725 + t68727 + t68730 + t68733;
    t73266
}
