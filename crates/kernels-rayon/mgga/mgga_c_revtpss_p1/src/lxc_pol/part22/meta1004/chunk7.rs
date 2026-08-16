//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3435/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3435(t300: f64, t63975: f64, t64023: f64, t64068: f64, t64101: f64, t64146: f64, t64152: f64, t64324: f64, t64484: f64, t18898: f64, t3015: f64, t981: f64) -> (f64, f64) {
    let t64488 = t300 * (t63975 + t64023 + t64068 + t64101 + t64146 + t64152 + t64324 + t64484);
    let t64491 = 0.6233709278045326953e3_f64 * t981 * t18898 * t3015;
    (t64488, t64491)
}
