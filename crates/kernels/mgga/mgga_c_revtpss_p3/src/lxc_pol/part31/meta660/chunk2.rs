//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2237/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2237<F: Float>(t196: F, t197: F, t22525: F, t2035: F, t22496: F, t25082: F, t33651: F, t29576: F, t7235: F, t2014: F, t22475: F, t7312: F) -> (F, F, F, F) {
    let t109077 = t22525 * t196 * t197;
    let t109078 = t109077 * t2035;
    let t109081 = F::new(6.0) * t25082 * t33651 * t22496;
    let t109087 = F::new(2.0) * t7235 * t29576;
    let t109090 = F::new(2.0) * t2014 * t7312 * t22475;
    (t109078, t109081, t109087, t109090)
}
