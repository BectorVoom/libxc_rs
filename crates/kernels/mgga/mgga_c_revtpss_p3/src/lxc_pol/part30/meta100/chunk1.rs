//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 630/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk630<F: Float>(t2311: F, t77: F, t2252: F, t2260: F, t2263: F, t2292: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F) {
    let t2312 = t77 * t2311;
    let t2315 = -t2252 * t85 / F::new(12.0) - t2260 * t85 / F::new(12.0) - t2263 * t85 / F::new(6.0) - t608 * t641 / F::new(6.0) + t2292 * t85 / F::new(24.0) + t628 * t641 / F::new(12.0) + t71 * t2312 / F::new(24.0);
    (t2312, t2315)
}
