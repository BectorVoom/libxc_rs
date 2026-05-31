//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 203/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk203<F: Float>(t546: F, t555: F, t213: F) -> (F, F, F) {
    let t557 = t546 * t555;
    let t560 = F::cast_from(1.0_f64) + F::cast_from(0.65854491829355115987e0_f64) * t213 * t557;
    let t561 = F::cast_from(1.0_f64) / t560;
    (t557, t560, t561)
}
