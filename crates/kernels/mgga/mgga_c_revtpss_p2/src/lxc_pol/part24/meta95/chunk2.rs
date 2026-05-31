//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 547/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk547<F: Float>(t2846: F, t340: F, t992: F, t338: F) -> (F, F, F) {
    let t3037 = F::cast_from(0.11111111111111111111e-1_f64) * t2846;
    let t3056 = F::cast_from(1.0_f64) / t992 / t340;
    let t3057 = t338 * t3056;
    (t3037, t3056, t3057)
}
