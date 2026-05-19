//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 61/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk61<F: Float>(t45: F, t153: F, t79: F, t57: F, t82: F, zeta_threshold: F) -> F {
    let t151 = t45 <= zeta_threshold;
    let t154 = piecewise3::<F>(t151, t153, t79);
    let t155 = t57 <= zeta_threshold;
    let t156 = piecewise3::<F>(t155, t153, t82);
    let t157 = t154 + t156 - F::new(2.0);
    t157
}
