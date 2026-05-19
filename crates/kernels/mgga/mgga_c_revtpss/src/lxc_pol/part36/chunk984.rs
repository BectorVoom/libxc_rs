//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 984/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk984<F: Float>(t57: F, t22671: F, t22688: F, t4335: F, t5825: F, t637: F, t770: F, t23138: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t23146 = piecewise3::<F>(t155, F::new(0.0), -F::new(8.0) / F::new(27.0) * t637 * t22688 - F::new(2.0) / F::new(3.0) * t4335 * t5825 - F::new(2.0) / F::new(3.0) * t770 * t22671);
    let t23148 = t23138 / F::new(2.0) + t23146 / F::new(2.0);
    t23148
}
