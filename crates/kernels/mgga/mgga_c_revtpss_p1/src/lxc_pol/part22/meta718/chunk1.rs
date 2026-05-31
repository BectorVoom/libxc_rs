//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2754/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2754<F: Float>(t2629: F, t39742: F, t2567: F, t2576: F, t2582: F) -> (F, F) {
    let t39744 = F::cast_from(0.1301229756036208781e0_f64) * t2629 * t39742;
    let t39747 = F::cast_from(36.0_f64) * t2582 * t2567 * t2576;
    (t39744, t39747)
}
