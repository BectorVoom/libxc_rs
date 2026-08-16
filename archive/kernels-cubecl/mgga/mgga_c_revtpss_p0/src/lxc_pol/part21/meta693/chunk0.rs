//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2514/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2514<F: Float>(t12378: F, t300: F, t13062: F, t13064: F, t3172: F, t1247: F, t13075: F, t1209: F, t13126: F, t17708: F, t127: F, t12988: F, t12989: F, t371: F) -> (F, F, F, F, F) {
    let t45319 = t300 * t12378;
    let t45346 = t13062 * t3172 * t13064;
    let t45352 = t1247 * t3172 * t13075;
    let t45371 = t1209 * t13126 * t17708;
    let t45382 = t12988 * t371 * t127 * t12989;
    (t45319, t45346, t45352, t45371, t45382)
}
