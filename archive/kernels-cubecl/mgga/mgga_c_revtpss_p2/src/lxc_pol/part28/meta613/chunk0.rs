//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2141/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2141<F: Float>(t13648: F, t2014: F, t7312: F, t25861: F, t7732: F, t2322: F, t28056: F, t25194: F, t7898: F, t25851: F, t10416: F, t7735: F) -> (F, F, F, F, F, F) {
    let t98597 = F::cast_from(2.0_f64) * t2014 * t7312 * t13648;
    let t98599 = F::cast_from(4.0_f64) * t7732 * t25861;
    let t98601 = F::cast_from(4.0_f64) * t2322 * t28056;
    let t98603 = F::cast_from(2.0_f64) * t7898 * t25194;
    let t98605 = F::cast_from(2.0_f64) * t7732 * t25851;
    let t98607 = F::cast_from(2.0_f64) * t10416 * t7735;
    (t98597, t98599, t98601, t98603, t98605, t98607)
}
