//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3435/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435<F: Float>(t300: F, t63975: F, t64023: F, t64068: F, t64101: F, t64146: F, t64152: F, t64324: F, t64484: F, t18898: F, t3015: F, t981: F) -> (F, F) {
    let t64488 = t300 * (t63975 + t64023 + t64068 + t64101 + t64146 + t64152 + t64324 + t64484);
    let t64491 = F::cast_from(0.6233709278045326953e3_f64) * t981 * t18898 * t3015;
    (t64488, t64491)
}
