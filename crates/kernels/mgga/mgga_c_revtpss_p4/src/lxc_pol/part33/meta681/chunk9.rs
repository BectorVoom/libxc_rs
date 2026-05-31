//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2230/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2230<F: Float>(t109233: F, t109235: F, t109237: F, t109239: F, t109241: F, t109244: F, t109246: F, t109248: F, t109250: F, t109252: F, t109254: F, t109256: F, t111685: F, t111708: F) -> F {
    let t111790 = t109233 + t109235 + t109237 + t109239 + t109241 + t109244 + t109246 + t109248 + t109250 + t109252 + t109254 + t109256 + t111685 + F::cast_from(2.0_f64) * t111708;
    t111790
}
