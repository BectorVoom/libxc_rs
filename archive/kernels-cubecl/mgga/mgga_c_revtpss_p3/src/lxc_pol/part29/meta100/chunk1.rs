//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 604/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk604<F: Float>(t2311: F, t77: F, t2252: F, t2260: F, t2263: F, t2292: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> (F, F) {
    let t2312 = t77 * t2311;
    let t2315 = -t2252 * t85 / F::cast_from(12.0_f64) - t2260 * t85 / F::cast_from(12.0_f64) - t2263 * t85 / F::cast_from(6.0_f64) - t608 * t641 / F::cast_from(6.0_f64) + t2292 * t85 / F::cast_from(24.0_f64) + t628 * t641 / F::cast_from(12.0_f64) + t71 * t2312 / F::cast_from(24.0_f64);
    (t2312, t2315)
}
