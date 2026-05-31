//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2201/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2201<F: Float>(t109077: F, t2035: F, t22496: F, t25082: F, t33651: F, t29576: F, t7235: F, t2014: F, t22475: F, t7312: F, t2034: F, t73407: F) -> (F, F, F, F, F) {
    let t109078 = t109077 * t2035;
    let t109081 = F::cast_from(6.0_f64) * t25082 * t33651 * t22496;
    let t109087 = F::cast_from(2.0_f64) * t7235 * t29576;
    let t109090 = F::cast_from(2.0_f64) * t2014 * t7312 * t22475;
    let t109092 = t2014 * t2034 * t73407;
    (t109078, t109081, t109087, t109090, t109092)
}
